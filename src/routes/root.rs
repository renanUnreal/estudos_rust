
use actix_web::HttpResponse;
use serde_json::json;

pub async fn index() -> HttpResponse {
    HttpResponse::Ok().json(json!({ "mensagem": "Bem vindo a minha API Rust" }))
}
