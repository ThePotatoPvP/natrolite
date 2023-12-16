use super::basic as basic;
use super::aimed as aimed;
use super::RequestData;

use actix_web::web;

pub async fn handle_basic(data: web::Json<RequestData>) -> web::Json<serde_json::Value> {
    match data.command.as_str() {
        "greet" => basic::_greet(&data),
        "parrot" => basic::_parrot(&data),
        _ => super::default(),
    }
}

pub async fn handle_aimed(data: web::Json<RequestData>) -> web::Json<serde_json::Value> {
    match data.command.as_str() {
        "poke" => aimed::_poke(&data),
        _ => super::default(),
    }
}