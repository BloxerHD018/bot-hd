use serenity::model::application::ResolvedOption;
use serenity::builder::CreateCommand;
use serenity::all::{CreateInteractionResponseMessage, CreateInteractionResponse};

pub fn register() -> CreateCommand {
    CreateCommand::new("ping")
        .description("A ping command")
}

pub fn execute(_options: &[ResolvedOption]) -> CreateInteractionResponse {
    let response = CreateInteractionResponseMessage::new()
        .content("Pong!")
        .ephemeral(true);

    CreateInteractionResponse::Message(response)
}