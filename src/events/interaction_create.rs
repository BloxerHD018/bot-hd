use serenity::{
    all::CreateInteractionResponseFollowup,
    model::application::Interaction,
    prelude::*
};

use crate::{commands, BotState};

pub async fn handle(ctx: Context, interaction: Interaction) {
    if let Interaction::Command(command) = interaction {
        let _ = command.defer_ephemeral(&ctx.http).await;
        let content = match command.data.name.as_str() { // Add new commands here

            "note" => Some({
                let data = ctx.data.read().await;
                let state = data.get::<BotState>().expect("Expected BotState").clone();
                commands::note::execute(&command.data.options(), command.clone(), ctx.clone(), state.db.clone()).await
            }),
            "ping" => Some(commands::ping::execute(&command.data.options())),

            
            _ => Some(CreateInteractionResponseFollowup::new().content("ERROR: Command not implemented")),
        };

        if let Some(content) = content {
            if let Err(why) = command.create_followup(&ctx.http, content).await {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    };
}