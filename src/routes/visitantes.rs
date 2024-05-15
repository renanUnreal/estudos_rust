use actix_web::{web, HttpResponse, Responder};


pub async fn index_all() -> impl Responder {
    println!("Acessou a rota '/visitantes' (retorna todos os visitantes)");
    HttpResponse::Ok().body("Lista de todos os visitantes")
}
pub async fn index_by_id(info: web::Path<(String,)>) -> impl Responder {
    let id = info.into_inner().0;
    println!("Acessou a rota '/visitantes/{}' (retorna o visitante com ID {})", id, id);
    HttpResponse::Ok().body(format!("Visitante com ID {}", id))
}
pub async fn create() -> impl Responder {
    println!("Acessou a rota '/visitantes/new' (cria um novo visitante)");
    HttpResponse::Ok().body("Criar um novo visitante")
}
pub async fn delete(info: web::Path<(String,)>) -> impl Responder {
    let id = info.into_inner().0;
    println!("Acessou a rota '/visitantes/delete/{}' (deleta o visitante com ID {})", id, id);
    HttpResponse::Ok().body(format!("Deletar visitante com ID {}", id))
}
pub async fn update(info: web::Path<(String,)>) -> impl Responder {
    let id = info.into_inner().0;
    println!("Acessou a rota '/visitantes/update/{}' (atualiza o visitante com ID {})", id, id);
    HttpResponse::Ok().body(format!("Atualizar visitante com ID {}", id))
}