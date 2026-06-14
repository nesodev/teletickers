use crate::domain::Notificacion;
use crate::error::{AppError, AppResult};
use crate::ports::{CompraPort, EventoPort, NotificacionPort, NotificationPort, UserPort};
use std::sync::Arc;
use uuid::Uuid;

pub struct SendPurchaseNotificationUseCase {
    compra_repo: Arc<dyn CompraPort>,
    evento_repo: Arc<dyn EventoPort>,
    user_repo: Arc<dyn UserPort>,
    notificacion_repo: Arc<dyn NotificacionPort>,
    notification_service: Arc<dyn NotificationPort>,
}

impl SendPurchaseNotificationUseCase {
    pub fn new(
        compra_repo: Arc<dyn CompraPort>,
        evento_repo: Arc<dyn EventoPort>,
        user_repo: Arc<dyn UserPort>,
        notificacion_repo: Arc<dyn NotificacionPort>,
        notification_service: Arc<dyn NotificationPort>,
    ) -> Self {
        Self {
            compra_repo,
            evento_repo,
            user_repo,
            notificacion_repo,
            notification_service,
        }
    }

    pub async fn execute(&self, compra_id: Uuid) -> AppResult<()> {
        let compra = self.compra_repo.find_by_id(compra_id).await?.ok_or(AppError::PurchaseNotFound)?;

        let evento = self.evento_repo.find_by_id(compra.evento_id).await?.ok_or(AppError::EventNotFound)?;

        let user = self.user_repo.find_by_id(compra.usuario_id).await?.ok_or(AppError::NotFound("Usuario".to_string()))?;

        let notificacion = Notificacion::compra_exitosa(user.id, evento.id, &evento.titulo);

        self.notificacion_repo.create(notificacion).await?;

        let email_body = format!(
            "Hola {},\n\nTu compra para el evento '{}' ha sido confirmada.\n\nGracias por tu preferencia.",
            user.nombre, evento.titulo
        );

        self.notification_service.send_email(user.email.value(), "Compra Confirmada - Ticky", &email_body).await?;

        Ok(())
    }
}
