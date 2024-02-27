use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::client::Context;
use serenity::model::prelude::Message;

#[command]
pub async fn hello(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Hello, example_plugin!").await?;

    Ok(())
}
