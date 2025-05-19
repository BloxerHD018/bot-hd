use serenity::prelude::*;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::sync::Arc;
use std::env;
use dotenvy::dotenv;

mod handler;
mod events;
mod commands;

// CREATE TABLE bot_hd_notes (name TEXT PRIMARY KEY, note TEXT NOT NULL, owner BIGINT NOT NULL);

pub struct BotState {
    pub db: Pool<Postgres>,
}

impl TypeMapKey for BotState {
    type Value = Arc<BotState>;
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").unwrap();
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await.unwrap();

    let state = Arc::new(BotState { db: db_pool });

    let token = env::var("DISCORD_TOKEN").expect("No token found when logging into bot");

    let intents = GatewayIntents::GUILD_MESSAGES
    | GatewayIntents::MESSAGE_CONTENT
    | GatewayIntents::GUILDS;

    let mut client = Client::builder(&token, intents)
        .event_handler(handler::Handler)
        .await
        .expect("Error creating Client");

    {
        let mut data = client.data.write().await;
        data.insert::<BotState>(state.clone());
    }

    if let Err(why) = client.start().await {
        println!("Client Error: {:?}", why)
    }
}
