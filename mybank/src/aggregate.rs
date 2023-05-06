use async_trait::async_trait;
use cqrs_es::{Aggregate};
use serde::{Serialize, Deserialize};

use crate::{commands::BankAccountCommand, domain_events::BankAccountEvent, errors::BankAccountError, services::BankAccountServices};

#[derive(Serialize, Default, Deserialize)]
pub struct BankAccount {
  account_id: String,
  balance: f64, // example purposes only - dont use float in real life
}

#[async_trait]
impl Aggregate for BankAccount {
    type Command = BankAccountCommand;
    type Event = BankAccountEvent;
    type Error = BankAccountError;
    type Services = BankAccountServices;

    fn aggregate_type() -> String {
        "Account".to_string()
    }

    async fn handle(
      &self,
      command: Self::Command,
      services: &Self::Services,
  ) -> Result<Vec<Self::Event>, Self::Error> {
      match command {
        BankAccountCommand::OpenAccount { account_id } => {
          Ok(vec![BankAccountEvent::AcountOpened { account_id }])
        },
        BankAccountCommand::DepositMoney { amount } => {
        let balance = self.balance + amount;
        Ok(vec![BankAccountEvent::CustomerDepositedMoney{
          amount,
          balance,
        }])
      },
      BankAccountCommand::WithdrawMoney { amount } => {
        let balance = self.balance - amount;
        if balance < 0_f64 {
          return Err(BankAccountError::new("funds not available"));
      }
        Ok(vec![BankAccountEvent::CustomerWithdrewCash {
          amount,
          balance,
        }])
      },
      BankAccountCommand::WriteCheck { check_number, amount } => {
        let balance = self.balance - amount;
        Ok(vec![BankAccountEvent::CustomerWroteCheck {
          check_number,
          amount,
          balance,
        }])
      }
        _ => todo!()
    }
  }


    fn apply(&mut self, event: Self::Event) {
        match event {
          BankAccountEvent::AcountOpened { account_id } => {
            self.account_id = account_id
          },
          BankAccountEvent::CustomerDepositedMoney { amount: _, balance } => {
            self.balance = balance
          },
          BankAccountEvent::CustomerWithdrewCash { amount: _, balance } => {
            self.balance = balance
          },
          BankAccountEvent::CustomerWroteCheck {
            check_number: _,
            amount: _,
            balance
          } => {
            self.balance = balance
          },
        }
    }
}