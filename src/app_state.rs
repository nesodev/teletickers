use crate::{
    config::{DatabaseConfig, JwtConfig, NotificacionConfig, PaymentConfig, StorageConfig},
    external::{email_service::EmailService, payment_gateway::PaymentGatewayService, storage_service::SupabaseStorageService},
    graphql::{context::GraphQLContext, schema::TickySchema},
    infrastructure::{
        auth::{jwt_service::JwtService, password_hasher::PasswordHasherService},
        db::connection::establish_connection,
    },
    ports::*,
    repositories::*,
};
use std::sync::Arc;

pub struct AppState {
    pub schema: TickySchema,
    pub context: Arc<GraphQLContext>,
    pub jwt_service: Arc<JwtService>,
    pub compra_repo: Arc<dyn CompraPort>,
    pub entrada_repo: Arc<dyn EntradaPort>,
}

impl AppState {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let db_config = DatabaseConfig::from_env();
        let jwt_config = JwtConfig::from_env();
        let payment_config = PaymentConfig::from_env();
        let storage_config = StorageConfig::from_env();
        let notificacion_config = NotificacionConfig::from_env();

        let db = establish_connection(&db_config).await?;

        let jwt_service = Arc::new(JwtService::new(jwt_config));
        let password_hasher = Arc::new(PasswordHasherService::new());
        let payment_service = Arc::new(PaymentGatewayService::new(payment_config));
        let storage_service = Arc::new(SupabaseStorageService::new(storage_config));
        let email_service = Arc::new(EmailService::new(notificacion_config));

        let user_repo = Arc::new(user_repo::UserRepository::new(db.clone())) as Arc<dyn UserPort>;
        let evento_repo = Arc::new(evento_repo::EventoRepository::new(db.clone())) as Arc<dyn EventoPort>;
        let tipo_entrada_repo = Arc::new(tipo_entrada_repo::TipoEntradaRepository::new(db.clone())) as Arc<dyn TipoEntradaPort>;
        let entrada_repo = Arc::new(entrada_repo::EntradaRepository::new(db.clone())) as Arc<dyn EntradaPort>;
        let compra_repo = Arc::new(compra_repo::CompraRepository::new(db.clone())) as Arc<dyn CompraPort>;
        let comprobante_repo = Arc::new(comprobante_repo::ComprobanteRepository::new(db.clone())) as Arc<dyn ComprobantePort>;
        let notificacion_repo = Arc::new(notificacion_repo::NotificacionRepository::new(db.clone())) as Arc<dyn NotificacionPort>;

        let context = Arc::new(GraphQLContext::new(
            user_repo,
            evento_repo,
            tipo_entrada_repo,
            compra_repo.clone(),
            entrada_repo.clone(),
            comprobante_repo,
            notificacion_repo,
            payment_service as Arc<dyn PaymentPort>,
            storage_service as Arc<dyn StoragePort>,
            email_service as Arc<dyn NotificationPort>,
        ));

        let schema = crate::graphql::schema::build_schema()
            .data(context.clone())
            .data(jwt_service.clone())
            .data(password_hasher)
            .finish();

        Ok(Self {
            schema,
            context,
            jwt_service,
            compra_repo,
            entrada_repo,
        })
    }
}
