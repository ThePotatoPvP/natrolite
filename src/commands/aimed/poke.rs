use actix_web::web;
use serde_json::json;

pub fn _poke(data: &super::super::RequestData) -> web::Json<serde_json::Value> {
    if let super::super::Args::Aimed(poke_data) = &data.arg {
        let response_message: String = format!("@{} pokes @{}!", data.author_id, poke_data.id);
        return web::Json(json!({"message": response_message}));
    }
    super::super::default()
}
