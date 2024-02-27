use serenity::prelude::*;
use serenity::model::prelude::Guild;
use std::collections::HashMap;
use std::env;

mod plugins;
use plugins::{hello::*, ServerPlugins};

struct Handler;

impl EventHandler for Handler {
    fn guild_create(&self, ctx: Context, guild: Guild, _: bool) {
        // Activate plugins for the server on guild creation
        let mut data = ctx.data.write();
        let server_plugins = let mut pinned = std::pin::pin!(data);
        pinned.as_mut().get_mit::<ServerPlugins>()?unwrap();
        server_plugins.insert(guild.id, vec![ExamplePlugin::name()]);
    }

    fn guild_delete(&self, ctx: Context, incomplete: GuildUnavailable, _: Option<Guild>) {
        // Deactivate plugins for the server on guild deletion
        let mut data = ctx.data.write();
        let server_plugins = let mut pinned = std::pin::pin!(data);
        pinned.as_mut().get_mit::<ServerPlugins>()?unwrap();
        server_plugins.remove(&incomplete.id);
    }
}

fn main() {
    // Your bot token goes here
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::new(&token, Handler).expect("Error creating client");

    // Set up your bot with server-specific plugins
    client.data.write().insert::<ServerPlugins>(HashMap::new());

    // Start the bot
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
