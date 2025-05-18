use serenity::{
    all::{CreateInteractionResponse, CreateInteractionResponseMessage}, model::application::Interaction, prelude::*
};

use crate::commands;

pub async fn handle(ctx: Context, interaction: Interaction) {
    if let Interaction::Command(command) = interaction {
        let content = match command.data.name.as_str() { // Add new commands here

            "ping" => Some(commands::ping::execute(&command.data.options())),

            
            _ => Some(CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content("ERROR: Not Implemented"))),
        };

        if let Some(content) = content {
            if let Err(why) = command.create_response(&ctx.http, content).await {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    };
}