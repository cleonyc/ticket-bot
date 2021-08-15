use crate::{handlers, DATABASE};
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};
#[command]
async fn delete(ctx: &Context, msg: &Message) -> CommandResult {
    {
        let db = DATABASE.get().unwrap().write().await;
        if !db
            .channel_is_ticket(msg.channel_id.0 as usize)
            .await
            .unwrap()
        {
            return Ok(());
        }
    };
    handlers::delete::delete(
        &ctx,
        msg.guild(&ctx.cache)
            .await
            .unwrap()
            .member(&ctx.http, msg.author.id)
            .await
            .unwrap(),
        ctx.cache.guild_channel(msg.channel_id).await.unwrap(),
    )
    .await?;

    Ok(())
}
