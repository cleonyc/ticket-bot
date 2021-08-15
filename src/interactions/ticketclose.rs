

use crate::{
    handlers::close::close,
};
use serenity::{
    client::Context,
    model::{
        interactions::{Interaction, InteractionResponseType, MessageComponent},
    },
};


pub async fn handle_close_interaction(
    ctx: &Context,
    interaction: &Interaction,
    comp: &MessageComponent,
) -> anyhow::Result<()> {
    if comp.custom_id != "tfticketclose" {
        return Ok(());
    }
    close(
        &ctx,
        interaction.member.clone().unwrap(),
        ctx.cache
            .guild_channel(interaction.channel_id.unwrap())
            .await
            .unwrap(),
    )
    .await?;
    interaction
        .create_interaction_response(&ctx.http, |r| {
            r.kind(InteractionResponseType::DeferredUpdateMessage)
        })
        .await?;
    Ok(())
}
