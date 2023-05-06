use async_trait::async_trait;
use cqrs_es::{Query, EventEnvelope, View, persist::GenericQuery};
use log::debug;
use postgres_es::PostgresViewRepository;
use serde::{Deserialize, Serialize};

use crate::{aggregate::BankAccount, domain_events::BankAccountEvent};

pub struct SimpleLoggingQuery {}

#[async_trait]
impl Query<BankAccount> for SimpleLoggingQuery {
  async fn dispatch(&self, aggregate_id: &str, events: &[EventEnvelope<BankAccount>]) {
    for event in events {
      debug!("{} - {}\n{:#?}", aggregate_id, event.sequence, event.payload);
    }
  }
}

pub type AccountQuery = GenericQuery<
  PostgresViewRepository<BankAccountView, BankAccount>,
  BankAccountView,
  BankAccount,
>;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct BankAccountView {
  account_id: Option<String>,
  balance: f64,
  written_checks: Vec<String>,
  ledger: Vec<LedgerEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
struct LedgerEntry {
  description: String,
  amount: f64,
}

impl LedgerEntry {
  fn new(description: &str, amount: f64) -> LedgerEntry {
    LedgerEntry {
      description: description.to_string(),
      amount
    }
  }
}

impl View<BankAccount> for BankAccountView {
    fn update(&mut self, event: &cqrs_es::EventEnvelope<BankAccount>) {
        match &event.payload {
          BankAccountEvent::AcountOpened { account_id } => {
            self.account_id = Some(account_id.clone());
          },
          BankAccountEvent::CustomerDepositedMoney { amount, balance } => {
            self.ledger.push(LedgerEntry::new("deposit", *amount));
            self.balance = *balance;
          },
          BankAccountEvent::CustomerWithdrewCash { amount, balance } => {
            self.ledger.push(LedgerEntry::new("withdrawal", *amount));
            self.balance = *balance;
          }
          BankAccountEvent::CustomerWroteCheck { check_number, amount, balance } => {
            self.ledger.push(LedgerEntry::new(check_number.clone().as_str(), *amount));
            self.written_checks.push(check_number.clone());
            self.balance = *balance;

          },
        }
    }
}