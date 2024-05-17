use actix_web::{web, HttpResponse, Responder};


pub async fn stands_index_all() -> impl Responder {
    println!("Acessou a rota '/stands' (retorna todos os stands)");
    HttpResponse::Ok().body("Lista de todos os stands")
}
pub async fn stands_index_by_id(info: web::Path<(String,)>) -> impl Responder {
    let id = info.into_inner().0;
    println!("Acessou a rota '/stands/{}' (retorna o stands com ID {})", id, id);
    HttpResponse::Ok().body(format!("stands com ID {}", id))
}
pub async fn stands_create() -> impl Responder {
    println!("Acessou a rota '/stands/new' (cria um novo stands)");
    HttpResponse::Ok().body("Criar um novo stands")
}
pub async fn stands_delete(info: web::Path<(String,)>) -> impl Responder {
    let id = info.into_inner().0;
    println!("Acessou a rota '/stands/delete/{}' (deleta o stands com ID {})", id, id);
    HttpResponse::Ok().body(format!("Deletar stands com ID {}", id))
}
pub async fn stands_update(info: web::Path<(String,)>) -> impl Responder {
    let id = info.into_inner().0;
    println!("Acessou a rota '/stands/update/{}' (atualiza o stands com ID {})", id, id);
    HttpResponse::Ok().body(format!("Atualizar stands com ID {}", id))
}