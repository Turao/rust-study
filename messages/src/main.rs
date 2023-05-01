use simplelog::{TermLogger, LevelFilter, Config, TerminalMode, ColorChoice};
use log::{debug, error};

mod api;
mod entities;
mod repositories;
mod services;

fn main() {
    TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    ).expect("logger init");

    // users
    let users = repositories::user::Repository::new().unwrap();
    let mut user_service = services::user::Service::new(users).unwrap();
    let user_id: i64;

    match user_service.create_user(api::users::CreateUserRequest {
        username: "john".to_string(),
    }) {
        Ok(result) => {
            debug!("{:?}", result);
            user_id = result.id;
        },
        Err(error) => {
            error!("{error}");
            return
        },
    }

    match user_service.get_by_id(api::users::GetByIDRequest {
        id: user_id,
    }) {
        Ok(response) => {
            debug!("{:?}", response)
        },
        Err(error) => {
            error!("{error}");
            return
        },
    }


    // messages
    let messages = repositories::message::Repository::new().unwrap();
    let mut message_service = services::message::Service::new(messages).unwrap();

    let message_id: i64;

    match message_service.create_message(api::messages::CreateMessageRequest{
        content: "this is my message".to_string(),
        author: user_id,
    }) {
        Ok(result) => {
            debug!("{:?}", result);
            message_id = result.id;
        },
        Err(error) => {
            error!("{error}");
            return
        }
    }

    match message_service.get_by_id(api::messages::GetByIDRequest{
        id: message_id,
    }) {
        Ok(result) => {
            match result.message {
                Some(message) => debug!("{:?}", message),
                None => debug!("found none")
            }
        },
        Err(error) => debug!("{}", error)
    }
}
