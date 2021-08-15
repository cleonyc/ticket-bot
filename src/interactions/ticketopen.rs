use std::path::Path;

use crate::{
    handlers::open,
    utils::{database::TicketStatus, files::write_messages},
    DATABASE, SETTINGS,
};
use serenity::{
    client::Context,
    model::{
        channel::{
            ChannelType, GuildChannel, PermissionOverwrite, PermissionOverwriteType, ReactionType,
        },
        id::ChannelId,
        interactions::{ButtonStyle, Interaction, InteractionResponseType, MessageComponent},
        Permissions,
    },
    prelude::Mentionable,
};
use uuid::Uuid;

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
