use std::fmt::{Formatter, Display};

#[derive(Debug, PartialEq)]
pub struct BankAccountError(String);

impl BankAccountError {
  pub fn new(message: &str) -> Self {
    BankAccountError(message.to_string())
  }
}

impl Display for BankAccountError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
      write!(f,"{}",self.0)
  }
}

impl std::error::Error for BankAccountError {}

impl From<&str> for BankAccountError {
    fn from(message: &str) -> Self {
        BankAccountError(message.to_string())
    }
}