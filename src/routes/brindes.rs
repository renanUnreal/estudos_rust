use actix_web::{web, HttpResponse, Responder};


pub async fn brindes_index_all() -> impl Responder {
    println!("Acessou a rota '/brindes' (retorna todos os brindes)");
    HttpResponse::Ok().body("Lista de todos os brindes")
}
pub async fn brindes_index_by_id(info: web::Path<(String,)>) -> impl Responder {
    let id = info.into_inner().0;
    println!("Acessou a rota '/brindes/{}' (retorna o brindes com ID {})", id, id);
    HttpResponse::Ok().body(format!("brindes com ID {}", id))
}
pub async fn brindes_create() -> impl Responder {
    println!("Acessou a rota '/brindes/new' (cria um novo brindes)");
    HttpResponse::Ok().body("Criar um novo brindes")
}
pub async fn brindes_delete(info: web::Path<(String,)>) -> impl Responder {
    let id = info.into_inner().0;
    println!("Acessou a rota '/brindes/delete/{}' (deleta o brindes com ID {})", id, id);
    HttpResponse::Ok().body(format!("Deletar brindes com ID {}", id))
}
pub async fn brindes_update(info: web::Path<(String,)>) -> impl Responder {
    let id = info.into_inner().0;
    println!("Acessou a rota '/brindes/update/{}' (atualiza o brindes com ID {})", id, id);
    HttpResponse::Ok().body(format!("Atualizar brindes com ID {}", id))
}