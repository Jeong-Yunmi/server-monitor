// src/metrics/middleware.rs
use std::time::Instant;
use actix_web::dev::{Service, Transform};
use futures::future::LocalBoxFuture;
use prometheus::HistogramTimer;

pub struct MetricsMiddleware;

impl<S, B> Transform<S, ServiceRequest> for MetricsMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = MetricsMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(MetricsMiddlewareService {
            service,
            start: Instant::now(),
        })
    }
}

pub struct MetricsMiddlewareService<S> {
    service: S,
    start: Instant,
}

impl<S, B> Service<ServiceRequest> for MetricsMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let timer = METRICS.http_request_duration_seconds.start_timer();
        METRICS.http_requests_total.inc();

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            timer.observe_duration();
            Ok(res)
        })
    }
}