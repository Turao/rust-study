#[cfg(test)]
mod aggregate_tests {
  use crate::{domain_events::BankAccountEvent, services::BankAccountServices, commands::BankAccountCommand, aggregate::BankAccount, errors::BankAccountError};
  use cqrs_es::test::TestFramework;

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
}