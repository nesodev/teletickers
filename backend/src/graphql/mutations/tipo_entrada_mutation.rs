use crate::domain::TipoEntrada;
use crate::domain::value_objects::Money;
use crate::graphql::context::GraphQLContext;
use crate::usecases::entradas::CreateTipoEntradaUseCase;
use async_graphql::*;
use rust_decimal::Decimal;
use std::sync::Arc;

#[derive(Default)]
pub struct TipoEntradaMutation;

#[derive(SimpleObject)]
pub struct TipoEntradaObject {
    pub id: String,
    pub evento_id: String,
    pub nombre: String,
    pub precio: f64,
    pub max_por_compra: i32,
    pub cantidad_disponible: i32,
    pub descripcion: Option<String>,
}

impl From<TipoEntrada> for TipoEntradaObject {
    fn from(tipo: TipoEntrada) -> Self {
        Self {
            id: tipo.id.to_string(),
            evento_id: tipo.evento_id.to_string(),
            nombre: tipo.nombre,
            precio: tipo.precio.to_f64(),
            max_por_compra: tipo.max_por_compra,
            cantidad_disponible: tipo.cantidad_disponible,
            descripcion: tipo.descripcion,
        }
    }
}

#[Object]
impl TipoEntradaMutation {
    async fn create_tipo_entrada(
        &self,
        ctx: &Context<'_>,
        evento_id: String,
        nombre: String,
        precio: f64,
        max_por_compra: i32,
        cantidad_disponible: i32,
        descripcion: Option<String>,
    ) -> Result<TipoEntradaObject> {
        let context = ctx.data::<Arc<GraphQLContext>>()?;
        let evento_uuid = evento_id.parse().map_err(|_| Error::new("Invalid evento_id"))?;

        let precio_decimal = Decimal::from_f64_retain(precio).ok_or_else(|| Error::new("Invalid precio"))?;
        let precio_money = Money::new(precio_decimal).map_err(|e| Error::new(e.to_string()))?;

        let use_case = CreateTipoEntradaUseCase::new(context.tipo_entrada_repo.clone());

        let tipo_entrada = use_case
            .execute(evento_uuid, nombre, precio_money, max_por_compra, cantidad_disponible, descripcion)
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(tipo_entrada.into())
    }
}
