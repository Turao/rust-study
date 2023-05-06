mod aggregate;
mod commands;
mod domain_events;
mod errors;
mod services;
mod aggregate_tests;
mod queries;
mod application;

use std::collections::HashMap;

use log::LevelFilter;
use simplelog::{TermLogger, Config, TerminalMode, ColorChoice};

use crate::commands::BankAccountCommand;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    ).expect("logger init");

    let app = application::Application::new().await;
    
    let mut metadata = HashMap::new();
    metadata.insert("time".to_string(), chrono::Utc::now().to_rfc3339());

    app.cqrs.execute_with_metadata(
        "aggregate_id_000",
        BankAccountCommand::DepositMoney{
            amount: 200_f64,
        },
        metadata,
    ).await.unwrap();
}

