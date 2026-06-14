use crate::error::AppError;
use crate::infrastructure::auth::JwtService;
use actix_web::{
    Error, HttpMessage,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use futures_util::future::LocalBoxFuture;
use std::future::{Ready, ready};
use std::rc::Rc;

pub struct AuthMiddleware {
    jwt_service: Rc<JwtService>,
}

impl AuthMiddleware {
    pub fn new(jwt_service: JwtService) -> Self {
        Self { jwt_service: Rc::new(jwt_service) }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service: Rc::new(service),
            jwt_service: self.jwt_service.clone(),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
    jwt_service: Rc<JwtService>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let jwt_service = self.jwt_service.clone();
        let service = self.service.clone();

        Box::pin(async move {
            let auth_header = req.headers().get("Authorization");

            let token = match auth_header {
                Some(value) => {
                    let auth_str = value.to_str().map_err(|_| AppError::Unauthorized("Invalid authorization header".to_string()))?;

                    if !auth_str.starts_with("Bearer ") {
                        return Err(AppError::Unauthorized("Invalid token format".to_string()).into());
                    }

                    &auth_str[7..]
                }
                None => return Err(AppError::Unauthorized("Missing authorization header".to_string()).into()),
            };

            let claims = jwt_service.validate_token(token).map_err(|e| Error::from(e))?;

            req.extensions_mut().insert(claims);

            service.call(req).await
        })
    }
}
