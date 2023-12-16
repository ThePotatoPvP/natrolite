use actix_web::web;
use serde_json::json;

pub fn _greet(data: &super::super::RequestData) -> web::Json<serde_json::Value> {
    if let super::super::Args::Basic(greet_data) = &data.arg {
        let response_message = format!("Hello, {}!", greet_data.text);
        return web::Json(json!({"message": response_message}));
    }
    super::super::default()
}
