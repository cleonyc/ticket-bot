

use crate::{
    handlers::delete,
};
use serenity::{
    client::Context,
    model::{
        interactions::{Interaction, InteractionResponseType, MessageComponent},
    },
};


pub async fn handle_delete_interaction(
    ctx: &Context,
    interaction: &Interaction,
    comp: &MessageComponent,
) -> anyhow::Result<()> {
    if comp.custom_id != "tfticketdelete" {
        return Ok(());
    }
    delete::delete(
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
