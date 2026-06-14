use actix_web::{
    Error, HttpResponse,
    body::{BoxBody, EitherBody, MessageBody},
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use futures_util::future::LocalBoxFuture;
use serde_json::json;
use std::{
    collections::HashMap,
    future::{Ready, ready},
    rc::Rc,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

#[derive(Clone)]
struct RateLimitEntry {
    count: u32,
    window_start: Instant,
}

#[derive(Clone)]
pub struct RateLimiter {
    max_requests: u32,
    window_duration: Duration,
    storage: Arc<Mutex<HashMap<String, RateLimitEntry>>>,
}

pub struct RateLimiterService<S> {
    service: Rc<S>,
    limiter: RateLimiter,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window_seconds: u64) -> Self {
        Self {
            max_requests,
            window_duration: Duration::from_secs(window_seconds),
            storage: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn check_rate_limit(&self, key: &str) -> bool {
        let mut storage = self.storage.lock().unwrap();
        let now = Instant::now();

        if let Some(entry) = storage.get_mut(key) {
            if now.duration_since(entry.window_start) > self.window_duration {
                entry.count = 1;
                entry.window_start = now;
                true
            } else if entry.count < self.max_requests {
                entry.count += 1;
                true
            } else {
                false
            }
        } else {
            storage.insert(key.to_string(), RateLimitEntry { count: 1, window_start: now });
            true
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RateLimiter
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = Error;
    type InitError = ();
    type Transform = RateLimiterService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimiterService {
            service: Rc::new(service),
            limiter: self.clone(),
        }))
    }
}

impl<S, B> Service<ServiceRequest> for RateLimiterService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let limiter = self.limiter.clone();

        Box::pin(async move {
            let ip = req.connection_info().realip_remote_addr().unwrap_or("unknown").to_string();

            if !limiter.check_rate_limit(&ip) {
                let response = HttpResponse::TooManyRequests().json(json!({ "error": "Rate limit exceeded" })).map_into_right_body();
                return Ok(req.into_response(response));
            }

            let res = service.call(req).await?;
            Ok(res.map_into_left_body())
        })
    }
}
