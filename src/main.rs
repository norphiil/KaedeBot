mod commands;

use std::env;
use dotenv::dotenv;
use serenity::async_trait;
use serenity::builder::{
    CreateInteractionResponse,
    CreateInteractionResponseMessage,
};
use serenity::model::prelude::*;
use serenity::prelude::*;

mod utils;

use utils::database;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction.clone() {
            println!("Received command interaction: {command:#?}");
            let content = match command.data.name.as_str() {
                "setup" => {
                    commands::setup::run(&ctx, &command).await.unwrap();
                    None
                },
                "modal" => {
                    commands::modal::run(&ctx, &command).await.unwrap();
                    None
                },
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        for guild_status in ready.guilds.iter() {
            println!("register slash command");
            let _ = guild_status.id
            .set_commands(&ctx.http, vec![
                commands::setup::register(),
                commands::modal::register(),
            ])
            .await
            .expect("Failed to create guild slash command");
        }
    }

    // async fn message(&self, ctx: Context, msg: Message) {
    // }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Build our client.
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    database::Db::setup_database().await;
    
    // Finally, start a single shard, and start listening to events.
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}