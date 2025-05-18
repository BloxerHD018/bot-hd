use serenity::{model::channel::Message, prelude::*};

pub async fn handle(ctx: Context, msg: Message) {
    if msg.content.to_lowercase().contains("dumb") {
        if let Err(why) = msg.reply(&ctx.http, "Says you dummy stfu").await {
            println!("Error sending message: {:?}", why);
        }
    }
}
