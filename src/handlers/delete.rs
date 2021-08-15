use std::path::Path;

use crate::{
    utils::{database::TicketStatus, files::write_messages},
    DATABASE, SETTINGS,
};
use serenity::{
    client::Context,
    model::{
        channel::{
            ChannelType, GuildChannel, PermissionOverwrite, PermissionOverwriteType, ReactionType,
        },
        guild::Member,
        id::ChannelId,
        interactions::{ButtonStyle, Interaction, InteractionResponseType, MessageComponent},
        Permissions,
    },
    prelude::Mentionable,
};
use uuid::Uuid;

pub async fn delete(ctx: &Context, closer: Member, channel: GuildChannel) -> anyhow::Result<()> {
    {
        let db = DATABASE.get().unwrap().write().await;

        let ticket_id = db.ticket_id(channel.id.0 as usize).await?;
        if db.ticket_status(ticket_id).await? != TicketStatus::Closed {
            channel
                .send_message(&ctx.http, |m| {
                    m.content(format!(
                        "{}, in order to delete a ticket, it must be closed first.",
                        closer.mention()
                    ))
                })
                .await?;
            return Ok(());
        }
        db.set_ticket_status(ticket_id, TicketStatus::Deleted)
            .await?;
    };
    channel.delete(&ctx.http).await?;
    {
        let logint = SETTINGS.write().await.get_int("logs_channel")?;
        let lchannel = ChannelId(logint as u64);
        let name = channel.name;
        lchannel
            .send_message(&ctx.http, |m| {
                m.content(format!("`{}` was deleted!", name));
                m
            })
            .await?;
    }
    Ok(())
}
