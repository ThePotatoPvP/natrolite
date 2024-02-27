use serenity::framework::standard::macros::group;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::collections::HashMap;

pub mod hello;
// Import other plugins here

#[group]
pub struct Plugins;

pub struct ServerPlugins;

impl TypeMapKey for ServerPlugins {
    type Value = HashMap<GuildId, Vec<String>>;
}
