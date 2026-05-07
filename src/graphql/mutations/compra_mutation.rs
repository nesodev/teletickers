use crate::domain::{Compra, MetodoPago};
use crate::graphql::context::GraphQLContext;
use crate::usecases::compras::*;
use async_graphql::*;

#[derive(Default)]
pub struct CompraMutation;

#[derive(SimpleObject)]
pub struct CompraObject {
    pub id: String,
    pub usuario_id: String,
    pub evento_id: String,
    pub monto_total: f64,
    pub metodo_pago: String,
    pub estado_pago: String,
}

impl From<Compra> for CompraObject {
    fn from(compra: Compra) -> Self {
        Self {
            id: compra.id.to_string(),
            usuario_id: compra.usuario_id.to_string(),
            evento_id: compra.evento_id.to_string(),
            monto_total: compra.monto_total.to_f64(),
            metodo_pago: compra.metodo_pago.as_str().to_string(),
            estado_pago: compra.estado_pago.as_str().to_string(),
        }
    }
}

#[derive(SimpleObject)]
pub struct PaymentResponse {
    pub transaction_id: String,
    pub status: String,
    pub payment_url: Option<String>,
}

#[Object]
impl CompraMutation {
    async fn create_compra(&self, ctx: &Context<'_>, evento_id: String, tipo_entrada_id: String, cantidad: i32, metodo_pago: String) -> Result<CompraObject> {
        let context = ctx.data::<GraphQLContext>()?;
        let user_id = context.current_user_id.ok_or_else(|| Error::new("Unauthorized"))?;

        let evento_uuid = evento_id.parse().map_err(|_| Error::new("Invalid evento_id"))?;
        let tipo_uuid = tipo_entrada_id.parse().map_err(|_| Error::new("Invalid tipo_entrada_id"))?;
        let metodo = MetodoPago::from_str(&metodo_pago).ok_or_else(|| Error::new("Invalid metodo_pago"))?;

        let use_case = CreateCompraUseCase::new(context.compra_repo.clone(), context.tipo_entrada_repo.clone());

        let compra = use_case.execute(user_id, evento_uuid, tipo_uuid, cantidad, metodo).await.map_err(|e| Error::new(e.to_string()))?;

        Ok(compra.into())
    }

    async fn process_payment(&self, ctx: &Context<'_>, compra_id: String, email: String) -> Result<PaymentResponse> {
        let context = ctx.data::<GraphQLContext>()?;
        let id = compra_id.parse().map_err(|_| Error::new("Invalid UUID"))?;

        let use_case = ProcessPaymentUseCase::new(context.compra_repo.clone(), context.payment_service.clone());

        let response = use_case.execute(id, email).await.map_err(|e| Error::new(e.to_string()))?;

        Ok(PaymentResponse {
            transaction_id: response.transaction_id,
            status: response.status,
            payment_url: response.payment_url,
        })
    }

    async fn cancel_compra(&self, ctx: &Context<'_>, compra_id: String) -> Result<CompraObject> {
        let context = ctx.data::<GraphQLContext>()?;
        let id = compra_id.parse().map_err(|_| Error::new("Invalid UUID"))?;

        let use_case = CancelCompraUseCase::new(context.compra_repo.clone(), context.entrada_repo.clone());

        let compra = use_case.execute(id).await.map_err(|e| Error::new(e.to_string()))?;

        Ok(compra.into())
    }
}
