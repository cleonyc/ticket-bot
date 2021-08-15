use std::path::Path;

use crate::{
    utils::{database::TicketStatus, files::write_messages},
    DATABASE, SETTINGS,
};
use serenity::{
    client::Context,
    model::{
        channel::{
            GuildChannel, PermissionOverwrite, PermissionOverwriteType, ReactionType,
        },
        guild::Member,
        id::ChannelId,
        interactions::{ButtonStyle},
        Permissions,
    },
    prelude::Mentionable,
};


pub async fn close(ctx: &Context, closer: Member, channel: GuildChannel) -> anyhow::Result<()> {
    let messages = {
        let db = DATABASE.get().unwrap().write().await;

        let ticket_id = db.ticket_id(channel.id.0 as usize).await?;
        if db.ticket_status(ticket_id).await? != TicketStatus::Open {
            channel
                .send_message(&ctx.http, |m| {
                    m.content(format!(
                        "{}, in order to close a ticket, it must be open in the first place.",
                        closer.mention()
                    ))
                })
                .await?;
            return Ok(());
        }
        db.set_ticket_status(ticket_id, TicketStatus::Closed)
            .await?;
        db.get_messages(ticket_id).await?
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
    for user in users_to_exclude {
        channel
            .create_permission(
                &ctx.http,
                &PermissionOverwrite {
                    deny: Permissions::READ_MESSAGES,
                    allow: Permissions::empty(),
                    kind: PermissionOverwriteType::Member(user),
                },
            )
            .await?;
    }
    let file = write_messages(messages).await?;
    channel
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.description(format!(
                    "Ticket has been closed and automatically archived by {}.",
                    closer.mention()
                ));
                e
            });
            m.components(|c| {
                c.create_action_row(|a| {
                    a.create_button(|b| {
                        b.label("Delete");
                        b.emoji(ReactionType::Unicode("❌".to_string()));
                        b.custom_id("tfticketdelete");
                        b.style(ButtonStyle::Danger);
                        b
                    });
                    a.create_button(|b| {
                        b.label("Reopen");
                        b.emoji(ReactionType::Unicode("✅".to_string()));
                        b.custom_id("tfticketopen");
                        b.style(ButtonStyle::Secondary);
                        b
                    });
                    a
                });
                c
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
                m.content(format!("`{}` was closed and automatically archived!", name));
                m.add_file(Path::new(&file));
                m
            })
            .await?;
    }
    Ok(())
}
