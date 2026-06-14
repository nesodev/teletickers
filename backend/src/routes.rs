use crate::graphql::handlers::*;
use crate::graphql::rest::*;
use crate::routes::payment_webhook::payment_webhookx;
use actix_web::{guard, web};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/graphql").guard(guard::Post()).to(graphql_handler))
        .service(
            web::resource("/graphql/ws")
                .guard(guard::Get())
                .guard(guard::Header("upgrade", "websocket"))
                .to(graphql_subscription),
        )
        .service(web::resource("/playground").guard(guard::Get()).to(graphql_playground))
        .service(health_check)
        .service(payment_webhookx)
        .service(validate_qr)
        .route("/", web::get().to(redirect_to_playground));
}
