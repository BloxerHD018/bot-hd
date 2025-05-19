use std::env;

use serenity::{
    all::GuildId, http, model::gateway::Ready, prelude::*
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

    // Delete global commands -- TESTING ONLY DONT LEAVE ON
    let commands = http::Http::get_global_commands(&ctx.http).await.unwrap();
    for command in commands {
        let _ = http::Http::delete_global_command(&ctx.http, command.id).await;
        println!("Deleted {} global command", command.name);
    }

    let guild_commands = guild_id
        .set_commands(&ctx.http, vec![
            commands::ping::register(),
            commands::note::register(),
        ]).await;

    if let Err(e) = guild_commands {
        println!("ERROR: Can't set guild commands: {e}")
    };

    // Register Global Commands
    //let _ = Command::create_global_command(&ctx.http, commands::ping::register()).await;
}