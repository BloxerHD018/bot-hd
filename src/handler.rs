use serenity::{
    async_trait,
    model::{
        channel::Message,
        gateway::Ready,
        application::Interaction,
    },
    prelude::*,
};

use crate::events;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        events::message::handle(ctx, msg).await;
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        events::ready::handle(ctx, ready).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        events::interaction_create::handle(ctx, interaction).await;
    }
}