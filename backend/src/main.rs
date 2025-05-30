mod handlers;
mod models;
mod services;
mod utils;

use actix_files::Files;
use actix_web::{middleware::Logger, web, App, HttpServer};
use handlers::websocket::websocket_handler;
use services::game_manager::GameManager;
use std::sync::Arc;
use tokio::sync::Mutex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let game_manager = Arc::new(Mutex::new(GameManager::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(game_manager.clone()))
            .wrap(Logger::default())
            .service(web::scope("/api").route("/ws", web::get().to(websocket_handler)))
            // .service(
            //     actix_files::Files::new("/", "../target/dx/frontend/debug/web/public")
            //         .index_file("index.html"),
            // )
            .service(
                actix_files::Files::new("/", "/Users/landlord/Desktop/arcade/_PVT/tic-tac-toe/target/dx/frontend/debug/web/public")
                    .index_file("index.html"),
            )

    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
