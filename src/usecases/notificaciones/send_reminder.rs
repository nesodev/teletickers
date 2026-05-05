use crate::domain::Notificacion;
use crate::error::{AppError, AppResult};
use crate::ports::{CompraPort, EventoPort, NotificacionPort, NotificationPort, UserPort};
use crate::utils::date_utils::days_until;
use std::sync::Arc;
use uuid::Uuid;

pub struct SendReminderUseCase {
    compra_repo: Arc<dyn CompraPort>,
    evento_repo: Arc<dyn EventoPort>,
    user_repo: Arc<dyn UserPort>,
    notificacion_repo: Arc<dyn NotificacionPort>,
    notification_service: Arc<dyn NotificationPort>,
}

impl SendReminderUseCase {
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

    pub async fn execute(&self, evento_id: Uuid) -> AppResult<()> {
        let evento = self.evento_repo.find_by_id(evento_id).await?.ok_or(AppError::EventNotFound)?;

        let dias = days_until(evento.fecha);

        let compras = self.compra_repo.list_by_evento(evento_id).await?;

        for compra in compras {
            if !compra.esta_pagado() {
                continue;
            }

            let user = self.user_repo.find_by_id(compra.usuario_id).await?.ok_or(AppError::NotFound("Usuario".to_string()))?;

            let notificacion = Notificacion::recordatorio_evento(user.id, evento.id, &evento.titulo, dias as i32);

            self.notificacion_repo.create(notificacion).await?;

            let email_body = format!(
                "Hola {},\n\nRecordatorio: El evento '{}' será en {} días.\n\nFecha: {}\nHora: {}\n\n¡No te lo pierdas!",
                user.nombre, evento.titulo, dias, evento.fecha, evento.hora
            );

            self.notification_service.send_email(user.email.value(), "Recordatorio de Evento - Ticky", &email_body).await?;
        }

        Ok(())
    }
}
