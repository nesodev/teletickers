use crate::graphql::{context::GraphQLContext, schema::TickySchema};
use crate::infrastructure::auth::jwt_service::JwtService;
use actix_web::{HttpRequest, HttpResponse, Result, web};
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use std::sync::Arc;

pub async fn graphql_playground() -> Result<HttpResponse> {
    let source = playground_source(GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql/ws"));
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(source))
}

pub async fn graphql_handler(
    schema: web::Data<TickySchema>,
    ctx: web::Data<Arc<GraphQLContext>>,
    jwt_service: web::Data<Arc<JwtService>>,
    req: GraphQLRequest,
    http_req: HttpRequest,
) -> GraphQLResponse {
    let mut request = req.into_inner();

    let mut arc_ctx: Arc<GraphQLContext> = ctx.get_ref().clone();

    if let Some(auth_header) = http_req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                if let Ok(user_id) = jwt_service.get_ref().extract_user_id(token) {
                    let inner: &mut GraphQLContext = Arc::make_mut(&mut arc_ctx);
                    inner.current_user_id = Some(user_id);
                }
            }
        }
    }

    request = request.data(arc_ctx);
    schema.execute(request).await.into()
}

pub async fn graphql_subscription(schema: web::Data<TickySchema>, req: HttpRequest, payload: web::Payload) -> Result<HttpResponse> {
    GraphQLSubscription::new(schema.get_ref().clone()).start(&req, payload)
}

pub async fn redirect_to_playground() -> HttpResponse {
    HttpResponse::Found().append_header(("Location", "/playground")).finish()
}
