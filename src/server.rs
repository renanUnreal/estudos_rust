use crate::routes::routes::config;

use actix_web::{App, HttpServer};
#[actix_web::main]
pub async fn servidor() -> std::io::Result<()> {
    // Inicia o servidor HTTP
    HttpServer::new(|| {
        App::new()
            .configure(config) // Adiciona a rota principal
    })
    .bind("127.0.0.1:8080")? // Define o endereço e porta
    .run()
    .await
}