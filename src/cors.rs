use actix_cors::Cors;
use actix_web::http::header;

pub fn configure_cors(frontend_url: &str) -> Cors {
    Cors::default()
        .allowed_origin(frontend_url)
        .allowed_origin("http://localhost:4321")
        .allowed_origin("http://127.0.0.1:4321")
        .allowed_origin("http://localhost:3000")
        .allowed_origin("http://127.0.0.1:3000")
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
        .supports_credentials()
        .max_age(3600)
}
