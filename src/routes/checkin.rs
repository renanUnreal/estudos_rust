use actix_web::{web, HttpResponse, Responder};


pub async fn checkin_index_all() -> impl Responder {
    println!("Acessou a rota '/checkin' (retorna todos os checkin)");
    HttpResponse::Ok().body("Lista de todos os checkin")
}
pub async fn checkin_index_by_id(info: web::Path<(String,)>) -> impl Responder {
    let id = info.into_inner().0;
    println!("Acessou a rota '/checkin/{}' (retorna o checkin com ID {})", id, id);
    HttpResponse::Ok().body(format!("checkin com ID {}", id))
}
pub async fn checkin_create() -> impl Responder {
    println!("Acessou a rota '/checkin/new' (cria um novo checkin)");
    HttpResponse::Ok().body("Criar um novo checkin")
}
