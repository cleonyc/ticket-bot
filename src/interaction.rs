use crate::{
    interactions::{
        panel::handle_panel_interaction, ticketclose::handle_close_interaction,
        ticketdelete::handle_delete_interaction, ticketopen::handle_open_interaction,
    },
    DATABASE,
};
use serenity::{
    client::Context,
    model::{
        channel::{ChannelType, ReactionType},
        id::ChannelId,
        interactions::{ButtonStyle, Interaction, InteractionData},
    },
    prelude::Mentionable,
};
use uuid::Uuid;

pub async fn handle_interaction(ctx: &Context, interaction: Interaction) -> anyhow::Result<()> {
    match interaction.data.clone() {
        None => {
            return Ok(());
        }
        Some(inter) => match inter {
            InteractionData::ApplicationCommand(_) => {}
            InteractionData::MessageComponent(comp) => {
                handle_panel_interaction(&ctx, &interaction, &comp).await?;
                handle_close_interaction(&ctx, &interaction, &comp).await?;
                handle_delete_interaction(&ctx, &interaction, &comp).await?;
                handle_open_interaction(&ctx, &interaction, &comp).await?;
            }
        },
    }
    Ok(())
}
