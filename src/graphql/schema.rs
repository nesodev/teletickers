use crate::graphql::mutations::*;
use crate::graphql::queries::*;
use crate::graphql::subscriptions::*;
use async_graphql::*;

#[derive(MergedObject, Default)]
pub struct QueryRoot(UserQuery, EventoQuery, CompraQuery, EntradaQuery, NotificacionQuery, AnalyticsQuery);

#[derive(MergedObject, Default)]
pub struct MutationRoot(AuthMutation, EventoMutation, CompraMutation, TipoEntradaMutation, NotificacionMutation);

#[derive(MergedSubscription, Default)]
pub struct SubscriptionRoot(ColaVirtualSubscription);

pub type TickySchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

pub fn build_schema() -> SchemaBuilder<QueryRoot, MutationRoot, SubscriptionRoot> {
    Schema::build(QueryRoot::default(), MutationRoot::default(), SubscriptionRoot::default())
}
