use crate::domain::Entrada;
use crate::graphql::context::GraphQLContext;
use async_graphql::*;

#[derive(Default)]
pub struct EntradaQuery;

#[derive(SimpleObject)]
pub struct EntradaObject {
    pub id: String,
    pub tipo_entrada_id: String,
    pub compra_id: String,
    pub qr_code: String,
    pub estado: String,
}

impl From<Entrada> for EntradaObject {
    fn from(entrada: Entrada) -> Self {
        Self {
            id: entrada.id.to_string(),
            tipo_entrada_id: entrada.tipo_entrada_id.to_string(),
            compra_id: entrada.compra_id.to_string(),
            qr_code: entrada.qr_code.value().to_string(),
            estado: entrada.estado.as_str().to_string(),
        }
    }
}

#[Object]
impl EntradaQuery {
    async fn mis_entradas(&self, ctx: &Context<'_>, compra_id: String) -> Result<Vec<EntradaObject>> {
        let context = ctx.data::<GraphQLContext>()?;
        let id = compra_id.parse().map_err(|_| Error::new("Invalid UUID"))?;

        let entradas = context.entrada_repo.list_by_compra(id).await.map_err(|e| Error::new(e.to_string()))?;

        Ok(entradas.into_iter().map(|e| e.into()).collect())
    }
}
