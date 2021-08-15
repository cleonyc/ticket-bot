use crate::{
    interactions::{
        panel::handle_panel_interaction, ticketclose::handle_close_interaction,
        ticketdelete::handle_delete_interaction, ticketopen::handle_open_interaction,
    },
};
use serenity::{
    client::Context,
    model::{
        interactions::{Interaction, InteractionData},
    },
};


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
