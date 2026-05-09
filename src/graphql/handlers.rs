use crate::graphql::{context::GraphQLContext, schema::TickySchema};
use actix_web::{HttpRequest, HttpResponse, Result, web};
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use std::sync::Arc;

pub async fn graphql_playground() -> Result<HttpResponse> {
    let source = playground_source(GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql/ws"));
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(source))
}

pub async fn graphql_handler(schema: web::Data<TickySchema>, ctx: web::Data<Arc<GraphQLContext>>, req: GraphQLRequest, http_req: HttpRequest) -> GraphQLResponse {
    let mut request = req.into_inner();
    let context = (**ctx).clone();

    if let Some(auth_header) = http_req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                request = request.data(token.to_string());
            }
        }
    }

    request = request.data(context);
    schema.execute(request).await.into()
}

pub async fn graphql_subscription(schema: web::Data<TickySchema>, req: HttpRequest, payload: web::Payload) -> Result<HttpResponse> {
    GraphQLSubscription::new(schema.get_ref().clone()).start(&req, payload)
}

pub async fn redirect_to_playground() -> HttpResponse {
    HttpResponse::Found().append_header(("Location", "/playground")).finish()
}
