mod commands;

use actix_web::{web, App, HttpServer, Error};
use tokio::signal;
use serde_json::json;
use serde::{Serialize, Deserialize};

use commands::splitter::{handle_basic, handle_aimed};

#[derive(Debug, Serialize, Deserialize)]
pub enum Args {
    Basic(commands::basic::general::BasicCommandData),
    Aimed(commands::aimed::general::AimedCommandData),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestData {
    pub command: String,
    pub cmd_type: String,
    pub author_id: u32,
    pub arg: Args,
}

async fn handle_request(data: web::Json<RequestData>) -> web::Json<serde_json::Value> {
    println!("Received request: {:?}", data);

    // Perform different actions based on the command
    match data.cmd_type.as_str() {
        "basic" => handle_basic(data).await,
        "aimed" => handle_aimed(data).await,
        _ => default(),
    }
}

pub fn default() -> web::Json<serde_json::Value> {
    web::Json(json!({"message": "Unknown command"}))
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    // Your existing server setup code here

    // Graceful shutdown on receiving SIGINT (Ctrl+C) or SIGTERM signals
    let graceful_shutdown = signal::ctrl_c();

    let server = HttpServer::new(|| {
        App::new().service(web::resource("/api/command").route(web::post().to(handle_request)))
    })
    .bind("127.0.0.1:8080")?
    .run();

    // Await the shutdown signal
    tokio::select! {
        _ = graceful_shutdown => {
            // Stop the server gracefully
            std::process::exit(0);
        }
        _ = server => {}
    };

    Ok(())
}
