use actix_web::web;
use serde_json::json;

pub fn _parrot(data: &super::super::RequestData) -> web::Json<serde_json::Value> {
    if let super::super::Args::Basic(parrot_data) = &data.arg {
        let response_message = format!("{}", parrot_data.text);
        return web::Json(json!({"message": response_message}));
    }
    super::super::default()
}
