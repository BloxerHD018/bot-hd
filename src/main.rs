use serenity::prelude::*;
use std::env;
use dotenvy::dotenv;

mod handler;
mod events;
mod commands;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("No token found when logging into bot");

    let intents = GatewayIntents::GUILD_MESSAGES
    | GatewayIntents::MESSAGE_CONTENT
    | GatewayIntents::GUILDS;

    let mut client = Client::builder(&token, intents)
        .event_handler(handler::Handler)
        .await
        .expect("Error creating Client");

    if let Err(why) = client.start().await {
        println!("Client Error: {:?}", why)
    }
}
