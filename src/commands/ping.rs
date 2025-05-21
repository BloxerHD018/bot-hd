use serenity::model::application::ResolvedOption;
use serenity::builder::CreateCommand;
use serenity::all::CreateInteractionResponseFollowup;

pub fn register() -> CreateCommand {
    CreateCommand::new("ping")
        .description("A ping command")
}

pub fn execute(_options: &[ResolvedOption]) -> CreateInteractionResponseFollowup {
    CreateInteractionResponseFollowup::new().content("Pong!")
}