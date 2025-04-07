use std::{
    env,
    future::{ready, Ready},
    rc::Rc,
};

use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::AUTHORIZATION,
    Error, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use serde_json::json;

pub struct SallaWebhookAuthorization;

impl<S, B> Transform<S, ServiceRequest> for SallaWebhookAuthorization
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>; // update here
    type Error = Error;
    type InitError = ();
    type Transform = SallaWebhookAuthorizationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SallaWebhookAuthorizationMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct SallaWebhookAuthorizationMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for SallaWebhookAuthorizationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>; // update here
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        dotenv::dotenv().ok();
        // Fetch the secret token from the environment
        let secret = match env::var("SALLA_PLUGIN_WEBHOOK_TOKEN") {
            Ok(token) => token,
            Err(_) => {
                // Log the error and return an internal server error response
                eprintln!("SALLA_PLUGIN_WEBHOOK_TOKEN is not set in the environment");
                let (http_req, _) = req.into_parts();
                let res = HttpResponse::InternalServerError()
                    .json(json!({ "success": false, "error": "Internal server error" }));
                let service_res = ServiceResponse::new(http_req, res);
                return Box::pin(async move { Ok(service_res.map_into_right_body()) });
            }
        };

        // Extract the Authorization header
        let auth_header = req.headers().get(AUTHORIZATION);
        if let Some(auth_value) = auth_header {
            if let Ok(incoming_token) = auth_value.to_str() {
                let header_secret_key = incoming_token.replace("Bearer ", "");
                if header_secret_key.as_bytes().eq(secret.as_bytes()) {
                    let service = Rc::clone(&self.service);

                    return Box::pin(async move {
                        // Getting some data here (just demo code for async function)
                        let res = service.call(req).await?;
                        Ok(res.map_into_left_body())
                    });
                }
            }
        }

        let (http_req, _) = req.into_parts();
        let res =
            HttpResponse::Unauthorized().json(json!({"sucees":false, "error":"Unauthorized"}));

        let service_res = ServiceResponse::new(http_req, res);
        Box::pin(async move { Ok(service_res.map_into_right_body()) })
    }
}
