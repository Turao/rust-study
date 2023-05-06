use log::{error};
use std::sync::Arc;

use cqrs_es::Query;
use postgres_es::{PostgresViewRepository, default_postgress_pool, PostgresCqrs, postgres_cqrs};

use crate::{queries::{SimpleLoggingQuery, AccountQuery}, aggregate::BankAccount, services::BankAccountServices};

pub struct Application {
  pub cqrs: PostgresCqrs<BankAccount>,
}

impl Application {
  pub async fn new() -> Application {
    let pool = default_postgress_pool("postgresql://test:test@localhost:5432/test").await;
    
    // prepare a materialized view of the Bank Account, and a query to read it
    let bank_account_view_repository = Arc::new(PostgresViewRepository::new("bank_account_view", pool.clone()));
    let mut bank_account_view_query = AccountQuery::new(bank_account_view_repository);

    // also add an error handler for the view query in case something bad happens
    bank_account_view_query.use_error_handler(
      Box::new(|error| error!("{:?}", error))
    );

    // prepare queries
    let queries: Vec<Box<dyn Query<BankAccount>>> = vec![
      Box::new(SimpleLoggingQuery{}),
      Box::new(bank_account_view_query),
    ];

    // prepare command handlers
    let services = BankAccountServices;

    // prepare the full framework for the BankAccount aggregate
    let cqrs = postgres_cqrs(pool.clone(), queries, services);

    Application {
      cqrs,
    }
  }
}

