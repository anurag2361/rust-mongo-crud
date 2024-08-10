use crate::handlers::item_handlers::*;
use actix_web::web;

pub fn init_item_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/items")
            .route("", web::post().to(create_item))
            .route("", web::get().to(get_items))
            .route("/{id}", web::get().to(get_item))
            .route("/{id}", web::put().to(update_item))
            .route("/{id}", web::delete().to(delete_item)),
    );

    // Health check route
    cfg.route("/health", web::get().to(health_check));
}
