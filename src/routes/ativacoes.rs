use actix_web::{web, HttpResponse, Responder};


pub async fn ativacoes_index_all() -> impl Responder {
    println!("Acessou a rota '/ativacoes' (retorna todos os ativacoes)");
    HttpResponse::Ok().body("Lista de todos os ativacoes")
}
pub async fn ativacoes_index_by_id(info: web::Path<(String,)>) -> impl Responder {
    let id = info.into_inner().0;
    println!("Acessou a rota '/ativacoes/{}' (retorna o ativacoes com ID {})", id, id);
    HttpResponse::Ok().body(format!("ativacoes com ID {}", id))
}
pub async fn ativacoes_create() -> impl Responder {
    println!("Acessou a rota '/ativacoes/new' (cria um novo ativacoes)");
    HttpResponse::Ok().body("Criar um novo ativacoes")
}
pub async fn ativacoes_delete(info: web::Path<(String,)>) -> impl Responder {
    let id = info.into_inner().0;
    println!("Acessou a rota '/ativacoes/delete/{}' (deleta o ativacoes com ID {})", id, id);
    HttpResponse::Ok().body(format!("Deletar ativacoes com ID {}", id))
}
pub async fn ativacoes_update(info: web::Path<(String,)>) -> impl Responder {
    let id = info.into_inner().0;
    println!("Acessou a rota '/ativacoes/update/{}' (atualiza o ativacoes com ID {})", id, id);
    HttpResponse::Ok().body(format!("Atualizar ativacoes com ID {}", id))
}