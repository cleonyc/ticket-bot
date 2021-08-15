

use crate::{
    handlers::open,
};
use serenity::{
    client::Context,
    model::{
        interactions::{Interaction, InteractionResponseType, MessageComponent},
    },
};


pub async fn handle_open_interaction(
    ctx: &Context,
    interaction: &Interaction,
    comp: &MessageComponent,
) -> anyhow::Result<()> {
    if comp.custom_id != "tfticketopen" {
        return Ok(());
    }
    open::open(
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
    match interaction.message.clone().unwrap() {
        serenity::model::interactions::InteractionMessage::Regular(m) => {
            m.delete(&ctx.http).await?
        }
        serenity::model::interactions::InteractionMessage::Ephemeral(_) => {}
    }
    Ok(())
}
