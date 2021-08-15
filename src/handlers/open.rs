

use crate::{
    utils::{database::TicketStatus},
    DATABASE, SETTINGS,
};
use serenity::{
    client::Context,
    model::{
        channel::{
            GuildChannel, PermissionOverwrite, PermissionOverwriteType,
        },
        guild::Member,
        id::ChannelId,
        Permissions,
    },
    prelude::Mentionable,
};


pub async fn open(ctx: &Context, opener: Member, channel: GuildChannel) -> anyhow::Result<()> {
    {
        let db = DATABASE.get().unwrap().write().await;

        let ticket_id = db.ticket_id(channel.id.0 as usize).await?;
        if db.ticket_status(ticket_id).await? != TicketStatus::Closed {
            channel
                .send_message(&ctx.http, |m| {
                    m.content(format!(
                        "{}, in order to reopen a ticket, it must be closed first.",
                        opener.mention()
                    ))
                })
                .await?;
            return Ok(());
        }
        db.set_ticket_status(ticket_id, TicketStatus::Open).await?;
    };
    let mut users_to_exclude = vec![];
    for i in channel.clone().permission_overwrites {
        match i.kind.clone() {
            PermissionOverwriteType::Member(uid) => users_to_exclude.push(uid),
            _ => {
                continue;
            }
        }
    }
    // this probably should be stored in a database rather than doing this hacky thing
    // will break if manual user overides are added or if a bot decides to force an overide for itself
    // if this is an issue that I encounter a lot I'll add some protections so bots aren't affected

    for user in users_to_exclude {
        channel
            .create_permission(
                &ctx.http,
                &PermissionOverwrite {
                    allow: Permissions::READ_MESSAGES,
                    deny: Permissions::empty(),
                    kind: PermissionOverwriteType::Member(user),
                },
            )
            .await?;
    }
    channel
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.description(format!("Ticket reopened by {}", opener.mention()));
                e
            });
            m
        })
        .await?;
    {
        let logint = SETTINGS.write().await.get_int("logs_channel")?;
        let lchannel = ChannelId(logint as u64);
        let name = channel.name;
        lchannel
            .send_message(&ctx.http, |m| {
                m.content(format!("`{}` was reopened!", name));
                m
            })
            .await?;
    }
    Ok(())
}
