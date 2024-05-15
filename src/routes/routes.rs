use actix_web::web;

use super::visitantes::{create,index_all, index_by_id, delete, update};
use super::root::index;


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(index))
    )
    .service(
        web::resource("/visitantes/all")
            .route(web::get().to(index_all))
    )
    .service(
        web::resource("/visitantes/{id}")
            .route(web::get().to(index_by_id))
    )
    .service(
        web::resource("/visitantes/new")
            .route(web::post().to(create))
    )
    .service(
        web::resource("/visitantes/delete/{id}")
            .route(web::delete().to(delete))
    )
    .service(
        web::resource("/visitantes/update/{id}")
            .route(web::put().to(update))
    );
}

