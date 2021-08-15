use std::{collections::HashMap, sync::Arc};


use commands::{add::*, close::*, delete::*, open::*, panel::*};
use config::Config;
use dotenv::dotenv;
use handler::Handler;
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;
use serenity::{
    framework::{standard::macros::group, StandardFramework},
    Client,
};

use tokio::sync::RwLock;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use utils::config::init_config;

use crate::utils::database::Database;

mod commands;
mod handler;
mod handlers;
mod interaction;
mod interactions;
mod utils;

pub static DATABASE: OnceCell<Arc<RwLock<Database>>> = OnceCell::new();
lazy_static! {
    pub static ref SETTINGS: RwLock<Config> = RwLock::new(Config::default());
    pub static ref COOLDOWN: Arc<RwLock<HashMap<u64, u64>>> = Arc::new(RwLock::new(HashMap::new()));
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    init_config().await.unwrap();
    let db = Database::new().await.unwrap();
    DATABASE.set(Arc::new(RwLock::new(db))).expect("msg");
    let prefix = SETTINGS.write().await.get_str("prefix").unwrap();
    let framework = StandardFramework::new()
        .configure(|c| c.prefix(&prefix)) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to start the logger");
    // Login with a bot token from the environment
    let token = SETTINGS.write().await.get_str("token").unwrap();
    let app_id = SETTINGS.write().await.get_int("app_id").unwrap();

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .application_id(app_id as u64)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
#[group]
#[commands(add, close, delete, panel, open)]
struct General;
