use serenity::all::CommandInteraction;
use serenity::builder::CreateCommand;
use serenity::prelude::*;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("talk").description("Start Talking")
}
