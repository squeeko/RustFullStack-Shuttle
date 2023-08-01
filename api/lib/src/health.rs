use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};

// pub fn service(cfg: &mut ServiceConfig) {
//     cfg.service(health);
// }

// We are going to use the route method to add the endpoint manually

pub fn service(cfg: &mut ServiceConfig) {
    cfg.route("/health", web::get().to(health));
}

async fn health() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("version", "0.0.1"))
        .finish()
}
