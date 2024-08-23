use serenity::all::{
    CommandInteraction, CreateAttachment, CreateButton, CreateChannel, CreateInteractionResponse, CreateInteractionResponseMessage
};
use serenity::builder::{CreateCommand, CreateEmbed};
use serenity::futures::StreamExt;
use serenity::model::prelude::*;
use serenity::prelude::*;
use tokio::time::Duration;

use crate::utils::database::{self, Db};

async fn create_embed_message() -> CreateEmbed {
    CreateEmbed::new()
        .title("Kaede Setup Panel")
        .image("attachment://kaede.jpg")
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    
    interaction
        .create_response(
            &ctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::default()
                    .embed(create_embed_message().await)
                    .add_file(
                        CreateAttachment::path("src/ressources/kaede.jpg")
                            .await
                            .unwrap(),
                    )
                    .button(
                        CreateButton::new("create_master_channel")
                            .label("〸 Create Master Channel")
                            .style(ButtonStyle::Success),
                    ),
            ),
        )
        .await?;

    // Wait for multiple interactions
    let m: Message = interaction.get_response(ctx).await?;
    let mut interaction_stream = m
        .await_component_interaction(&ctx.shard)
        .timeout(Duration::from_secs(60 * 3))
        .stream();

    while let Some(interaction) = interaction_stream.next().await {
        match interaction.data.custom_id.as_str() {
            "create_master_channel" => {
                println!("create_master_channel !");
                let builder = CreateChannel::new("➕ New Channel").kind(ChannelType::Voice);
                let new_channel = interaction.guild_id.unwrap().create_channel(&ctx, builder).await?;
                
                {
                    println!("Creating database entry...");
                    let mut lock = database::get_instance().lock();
                    let db = lock.as_mut().unwrap().as_mut().unwrap();
                    println!("Creating database entry... done");
                    let move _futur = db.create_new_channel(new_channel.guild_id, new_channel.id, interaction.user.id, None)
                    _future.await;
                    println!("Creating database entry... finish");
                }

                interaction
                    .create_response(
                        &ctx,
                        CreateInteractionResponse::UpdateMessage(
                            CreateInteractionResponseMessage::default()
                                .embed(create_embed_message().await)
                                .add_file(
                                    CreateAttachment::path("src/ressources/kaede.jpg")
                                        .await
                                        .unwrap(),
                                )
                                .button(
                                    CreateButton::new("change_name")
                                        .label("Change Name")
                                        .style(ButtonStyle::Success),
                                ),
                        ),
                    )
                    .await?;
            }
            _ => {
                println!("Unknown button clicked !");
            }
        }
    }
    m.delete(&ctx).await.unwrap();

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("setup").description("A setup command")
}
