
mod router_html2pdf;
use std::rc::Rc;
use actix_web::web;
use actix_cors::Cors;
use actix_web::http::header;


pub fn register_advert_routes(cfg: &mut web::ServiceConfig) {
    let cors = get_cors();
    cfg.service(
        web::scope("")
            .wrap(cors.clone())
            .service(router_html2pdf::html2pdf_post)
    );
}


fn get_cors() -> Rc<Cors> {
    let cors = Cors::default()
        .allowed_methods(vec!["HEAD", "GET", "POST", "PUT", "OPTIONS", "DELETE"])
        .allowed_headers(vec![
            header::AUTHORIZATION,
            header::ACCEPT,
            header::CONTENT_TYPE,
        ])
        .allow_any_origin()
        .max_age(3600);
    Rc::new(cors)
}
