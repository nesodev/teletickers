use actix_web::{App, HttpServer, middleware::Logger, web};
use dotenvy::dotenv;
use ticky::{
    app_state::AppState,
    cors::configure_cors,
    graphql::{handlers::*, rest::*},
    server::ServerConfig,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("🚀 Iniciando Ticky Backend");

    let server_config = ServerConfig::from_env();
    let frontend_url = server_config.frontend_url.clone();
    let app_state = AppState::new().await.expect("Error al inicializar aplicación");

    log::info!("✅ Aplicación inicializada");
    log::info!("🌐 Servidor: {}", server_config.base_url());
    log::info!("🎮 Playground: {}/playground", server_config.base_url());
    log::info!("🔌 WebSocket: ws://{}/graphql/ws", server_config.address());

    HttpServer::new(move || {
        App::new()
            .wrap(configure_cors(&frontend_url))
            .wrap(Logger::default())
            .app_data(web::Data::new(app_state.schema.clone()))
            .app_data(web::Data::new(app_state.context.clone()))
            .app_data(web::Data::new(app_state.jwt_service.clone()))
            .app_data(web::Data::new(app_state.compra_repo.clone()))
            .app_data(web::Data::new(app_state.entrada_repo.clone()))
            .service(health_check)
            .service(payment_webhookx)
            .service(validate_qr)
            .service(web::resource("/graphql").route(web::post().to(graphql_handler)))
            .service(web::resource("/graphql/ws").route(web::get().to(graphql_subscription)))
            .service(web::resource("/playground").route(web::get().to(graphql_playground)))
            .route("/", web::get().to(redirect_to_playground))
    })
    .bind(server_config.address())?
    .run()
    .await
}
