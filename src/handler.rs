use crate::{interaction::handle_interaction, DATABASE};
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{channel::Message, event::ResumedEvent, interactions::Interaction, prelude::Ready},
};
use tracing::log::info;
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
    async fn message(&self, ctx: Context, msg: Message) {
        {
            let db = DATABASE.get().unwrap().write().await;
            if !db
                .channel_is_ticket(msg.channel_id.0 as usize)
                .await
                .unwrap()
            {
                return;
            }
        }

        let log_message = format!(
            "{} ({}) -> {}",
            msg.author.tag(),
            msg.author.id.0,
            msg.content_safe(&ctx.cache).await
        );

        let ticket_id = {
            let db = DATABASE.get().unwrap().write().await;
            db.ticket_id(msg.channel_id.0 as usize).await.unwrap()
        };
        {
            let db = DATABASE.get().unwrap().write().await;
            db.add_message(ticket_id, log_message).await.unwrap()
        }
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        handle_interaction(&ctx, interaction).await.unwrap();
    }
}
