use crate::ports::*;
use std::sync::Arc;
use uuid::Uuid;

pub struct GraphQLContext {
    pub user_repo: Arc<dyn UserPort>,
    pub evento_repo: Arc<dyn EventoPort>,
    pub tipo_entrada_repo: Arc<dyn TipoEntradaPort>,
    pub compra_repo: Arc<dyn CompraPort>,
    pub entrada_repo: Arc<dyn EntradaPort>,
    pub comprobante_repo: Arc<dyn ComprobantePort>,
    pub notificacion_repo: Arc<dyn NotificacionPort>,
    pub payment_service: Arc<dyn PaymentPort>,
    pub storage_service: Arc<dyn StoragePort>,
    pub notification_service: Arc<dyn NotificationPort>,
    pub current_user_id: Option<Uuid>,
}

impl GraphQLContext {
    pub fn new(
        user_repo: Arc<dyn UserPort>,
        evento_repo: Arc<dyn EventoPort>,
        tipo_entrada_repo: Arc<dyn TipoEntradaPort>,
        compra_repo: Arc<dyn CompraPort>,
        entrada_repo: Arc<dyn EntradaPort>,
        comprobante_repo: Arc<dyn ComprobantePort>,
        notificacion_repo: Arc<dyn NotificacionPort>,
        payment_service: Arc<dyn PaymentPort>,
        storage_service: Arc<dyn StoragePort>,
        notification_service: Arc<dyn NotificationPort>,
    ) -> Self {
        Self {
            user_repo,
            evento_repo,
            tipo_entrada_repo,
            compra_repo,
            entrada_repo,
            comprobante_repo,
            notificacion_repo,
            payment_service,
            storage_service,
            notification_service,
            current_user_id: None,
        }
    }

    pub fn with_user(mut self, user_id: Uuid) -> Self {
        self.current_user_id = Some(user_id);
        self
    }
}
