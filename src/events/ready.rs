use std::env;

use serenity::{
    all::GuildId,
    model::gateway::Ready,
    prelude::*
};

use crate::commands;

pub async fn handle(ctx: Context, ready: Ready) {
    println!("Connected to Discord as {}", ready.user.name);

    let guild_id = GuildId::new(
        env::var("TEST_GUILD_ID")
            .expect("TEST_GUILD_ID must be set when using guild commands")
            .parse()
            .expect("TEST_GUILD_ID must be an integer"),
    );

    let _guild_commands = guild_id
        .set_commands(&ctx.http, vec![
            commands::ping::register(),
        ]).await;

    // Register Global Commands
    //let _ = Command::create_global_command(&ctx.http, commands::ping::register()).await;
}