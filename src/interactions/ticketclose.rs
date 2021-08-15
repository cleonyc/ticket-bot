use std::path::Path;

use crate::{
    handlers::close::close,
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
