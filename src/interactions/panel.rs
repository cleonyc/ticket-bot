

use crate::{COOLDOWN, DATABASE, SETTINGS};
use serenity::{
    client::Context,
    model::{
        channel::{ChannelType, PermissionOverwrite, PermissionOverwriteType, ReactionType},
        id::ChannelId,
        interactions::{ButtonStyle, Interaction, InteractionResponseType, MessageComponent},
        Permissions,
    },
    prelude::Mentionable,
};
use time::OffsetDateTime;
use uuid::Uuid;

pub async fn handle_panel_interaction(
    ctx: &Context,
    interaction: &Interaction,
    comp: &MessageComponent,
) -> anyhow::Result<()> {
    {
        let config = SETTINGS.write().await;
        let cooldown = COOLDOWN.read().await;
        let uold = cooldown
            .get(
                &interaction.member.clone().unwrap().user.id.0,
            )
            .unwrap_or(&(0 as u64)) ;
        if *uold as i64 + config.get_int("cooldown").unwrap() > OffsetDateTime::now_utc().unix_timestamp() {
            return Ok(());
        }
    }
    let db = DATABASE.get().unwrap().write().await;
    let id = {
        match Uuid::parse_str(&comp.custom_id) {
            Ok(id) => {
                // println!("ok");
                if !db.is_panel(id).await? {
                    // println!("ok but");
                    return Ok(());
                }
                id
            }
            Err(_) => return Ok(()),
        }
    };
    let cat = ChannelId(db.get_category(id).await? as u64);
    // let perms = vec![

    //     PermissionOverwrite {
    //         allow: Permissions::SEND_MESSAGES,
    //         deny: Permissions::empty(),
    //         kind: PermissionOverwriteType::Member(interaction.member.clone().unwrap().user.id),
    //     },
    // ];
    let mut new_channel = interaction
        .guild_id
        .unwrap()
        .create_channel(&ctx.http, |c| {
            c.name("new-ticket");
            c.category(cat);
            c.kind(ChannelType::Text);
            c
        })
        .await?;
    new_channel
        .create_permission(
            &ctx.http,
            &PermissionOverwrite {
                allow: Permissions::READ_MESSAGES,
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Member(interaction.member.clone().unwrap().user.id),
            },
        )
        .await?;
    let ticket_id = db.open_ticket(new_channel.id.0 as usize).await?;
    new_channel
        .edit(&ctx.http, |c| c.name(format!("ticket-{}", ticket_id)))
        .await?;
    new_channel
        .send_message(&ctx.http, |m| {
            m.content(interaction.member.clone().unwrap().mention());
            m.embed(|e| {
                e.description(
                    "Support will be with you shortly!\n To close this ticket click on the button.",
                );
                e
            });
            m.components(|c| {
                c.create_action_row(|a| {
                    a.create_button(|b| {
                        b.label("Close");
                        b.emoji(ReactionType::Unicode("🔒".to_string()));
                        b.custom_id("tfticketclose");
                        b.style(ButtonStyle::Primary);
                        b
                    });
                    a
                });
                c
            });
            m
        })
        .await?;
    interaction
        .create_interaction_response(&ctx.http, |r| {
            r.kind(InteractionResponseType::DeferredUpdateMessage)
        })
        .await?;
    {
        let mut cooldown = COOLDOWN.write().await;
        cooldown
            .insert(
                interaction.member.clone().unwrap().user.id.0,
                OffsetDateTime::now_utc().unix_timestamp() as u64,
            );
    }
    Ok(())
}
