use actix_web::{web, HttpResponse, Responder};


pub async fn index_all() -> impl Responder {
    println!("Acessou a rota '/adm' (retorna todos os adm)");
    HttpResponse::Ok().body("Lista de todos os adm")
}
pub async fn index_by_id(info: web::Path<(String,)>) -> impl Responder {
    let id = info.into_inner().0;
    println!("Acessou a rota '/adm/{}' (retorna o adm com ID {})", id, id);
    HttpResponse::Ok().body(format!("adm com ID {}", id))
}
pub async fn create() -> impl Responder {
    println!("Acessou a rota '/adm/new' (cria um novo adm)");
    HttpResponse::Ok().body("Criar um novo adm")
}
pub async fn delete(info: web::Path<(String,)>) -> impl Responder {
    let id = info.into_inner().0;
    println!("Acessou a rota '/adm/delete/{}' (deleta o adm com ID {})", id, id);
    HttpResponse::Ok().body(format!("Deletar adm com ID {}", id))
}
pub async fn update(info: web::Path<(String,)>) -> impl Responder {
    let id = info.into_inner().0;
    println!("Acessou a rota '/adm/update/{}' (atualiza o adm com ID {})", id, id);
    HttpResponse::Ok().body(format!("Atualizar adm com ID {}", id))
}