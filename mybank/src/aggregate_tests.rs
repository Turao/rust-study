#[cfg(test)]
mod aggregate_tests {
  use crate::{domain_events::BankAccountEvent, services::BankAccountServices, commands::BankAccountCommand, aggregate::{BankAccount}, errors::BankAccountError, queries::SimpleLoggingQuery};
  use cqrs_es::{test::TestFramework, CqrsFramework, mem_store::MemStore};

  type AccountTestFramework = TestFramework<BankAccount>;

  #[test]
  fn test_deposit_money() {
    let expected = BankAccountEvent::CustomerDepositedMoney {
      amount: 200.0,
      balance: 200.0,
    };

    AccountTestFramework::with(BankAccountServices)
      .given_no_previous_events()
      .when(BankAccountCommand::DepositMoney { amount: 200.0 })
      .then_expect_events(vec![expected]);
  }

  #[test]
  fn test_deposit_money_with_balance() {
    let previous = BankAccountEvent::CustomerDepositedMoney { amount: 200.0, balance: 200.0 };
    let expected = BankAccountEvent::CustomerDepositedMoney { amount: 200.0, balance: 400.0 };

    AccountTestFramework::with(BankAccountServices)
      .given(vec![previous])
      .when(BankAccountCommand::DepositMoney { amount: 200.0 })
      .then_expect_events(vec![expected])
  }

  #[test]
  fn test_withdraw_money() {
    let previous = BankAccountEvent::CustomerDepositedMoney { amount: 200.0, balance: 200.0 };
    let expected = BankAccountEvent::CustomerWithdrewCash { amount: 100.0, balance: 100.0 };

    AccountTestFramework::with(BankAccountServices)
      .given(vec![previous])
      .when(BankAccountCommand::WithdrawMoney { amount: 100.0 })
      .then_expect_events(vec![expected]);
  }

  #[test]
  fn test_withdraw_money_funds_not_available() {
    AccountTestFramework::with(BankAccountServices)
      .given_no_previous_events()
      .when(BankAccountCommand::WithdrawMoney { amount: 200.0 })
      .then_expect_error(BankAccountError::from("funds not available"));
  }

  #[tokio::test]
  async fn test_event_store() {
    let event_store = MemStore::<BankAccount>::default();
    let query = SimpleLoggingQuery {};
    let cqrs = CqrsFramework::new(
      event_store,
      vec![Box::new(query)],
      BankAccountServices{},
    );

    let aggregate_id = "aggregate_id_000";

    cqrs.execute(
      aggregate_id,
      BankAccountCommand::DepositMoney {
        amount: 1000_f64,
      }
    ).await.unwrap();

    cqrs.execute(
      aggregate_id,
      BankAccountCommand::WriteCheck {
        check_number: "check_no".to_string(),
        amount: 150.0,
      }
    ).await.unwrap();

  }
}