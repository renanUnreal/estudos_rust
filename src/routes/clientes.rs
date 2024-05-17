use actix_web::{web, HttpResponse, Responder};


pub async fn index_all() -> impl Responder {
    println!("Acessou a rota '/clientes' (retorna todos os clientes)");
    HttpResponse::Ok().body("Lista de todos os clientes")
}
pub async fn index_by_id(info: web::Path<(String,)>) -> impl Responder {
    let id = info.into_inner().0;
    println!("Acessou a rota '/clientes/{}' (retorna o clientes com ID {})", id, id);
    HttpResponse::Ok().body(format!("clientes com ID {}", id))
}
pub async fn create() -> impl Responder {
    println!("Acessou a rota '/clientes/new' (cria um novo clientes)");
    HttpResponse::Ok().body("Criar um novo clientes")
}
pub async fn delete(info: web::Path<(String,)>) -> impl Responder {
    let id = info.into_inner().0;
    println!("Acessou a rota '/clientes/delete/{}' (deleta o clientes com ID {})", id, id);
    HttpResponse::Ok().body(format!("Deletar clientes com ID {}", id))
}
pub async fn update(info: web::Path<(String,)>) -> impl Responder {
    let id = info.into_inner().0;
    println!("Acessou a rota '/clientes/update/{}' (atualiza o clientes com ID {})", id, id);
    HttpResponse::Ok().body(format!("Atualizar clientes com ID {}", id))
}