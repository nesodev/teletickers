use crate::graphql::context::GraphQLContext;
use crate::usecases::analytics::*;
use async_graphql::*;

#[derive(Default)]
pub struct AnalyticsQuery;

#[derive(SimpleObject)]
pub struct DashboardMetricsObject {
    pub total_eventos: i32,
    pub total_ventas: i64,
    pub ingresos_totales: f64,
    pub eventos_activos: i32,
}

#[derive(SimpleObject)]
pub struct EventoStatsObject {
    pub evento_id: String,
    pub titulo: String,
    pub total_ventas: i64,
    pub ingresos: f64,
    pub entradas_vendidas: i32,
    pub aforo: i32,
    pub porcentaje_ocupacion: f64,
}

#[Object]
impl AnalyticsQuery {
    async fn dashboard_metrics(&self, ctx: &Context<'_>) -> Result<DashboardMetricsObject> {
        let context = ctx.data::<GraphQLContext>()?;
        let user_id = context.current_user_id.ok_or_else(|| Error::new("Unauthorized"))?;

        let use_case = GetDashboardMetricsUseCase::new(context.evento_repo.clone(), context.compra_repo.clone());

        let metrics = use_case.execute(user_id).await.map_err(|e| Error::new(e.to_string()))?;

        Ok(DashboardMetricsObject {
            total_eventos: metrics.total_eventos as i32,
            total_ventas: metrics.total_ventas,
            ingresos_totales: metrics.ingresos_totales.to_f64(),
            eventos_activos: metrics.eventos_activos as i32,
        })
    }

    async fn evento_stats(&self, ctx: &Context<'_>, evento_id: String) -> Result<EventoStatsObject> {
        let context = ctx.data::<GraphQLContext>()?;
        let id = evento_id.parse().map_err(|_| Error::new("Invalid UUID"))?;

        let use_case = GetEventoStatsUseCase::new(context.evento_repo.clone(), context.compra_repo.clone(), context.entrada_repo.clone());

        let stats = use_case.execute(id).await.map_err(|e| Error::new(e.to_string()))?;

        Ok(EventoStatsObject {
            evento_id: stats.evento_id.to_string(),
            titulo: stats.titulo,
            total_ventas: stats.total_ventas,
            ingresos: stats.ingresos.to_f64(),
            entradas_vendidas: stats.entradas_vendidas as i32,
            aforo: stats.aforo,
            porcentaje_ocupacion: stats.porcentaje_ocupacion,
        })
    }
}
