use std::{
    future::{self, Future, Ready},
    pin::Pin,
    rc::Rc,
    task::{Context, Poll},
};

use actix_web::{
    body::MessageBody,
    dev::{Payload, Service, ServiceRequest, ServiceResponse, Transform},
    Error, FromRequest, HttpMessage, HttpRequest,
};
use serde::{Deserialize, Serialize};

/// actix extractor for jwt claims
#[derive(Debug, Serialize, Deserialize, Clone, Default, Hash)]
pub struct JwtClaims {
    pub user_id: i64,
    pub username: String,
    pub exp: usize,
}

impl FromRequest for JwtClaims {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        println!("==extractor called==");
        match req.extensions().get::<Self>() {
            Some(claims) => future::ready(Ok(claims.clone())),
            _ => future::ready(Ok(JwtClaims {
                username: format!(
                    "Set by extractor! current time: {}",
                    chrono::Utc::now().to_rfc3339()
                ),
                ..Default::default()
            })),
        }
    }
}

pub struct JwtMiddleware;

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: 'static + Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    B: MessageBody,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ready(Ok(JwtService {
            inner_service: Rc::new(service),
        }))
    }
}

pub struct JwtService<S> {
    inner_service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for JwtService<S>
where
    S: 'static + Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    B: MessageBody,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner_service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("==middleware called==");
        let inner_service = Rc::clone(&self.inner_service);
        Box::pin(async move {
            req.extensions_mut().insert(JwtClaims {
                username: format!(
                    "Set by middleware! current time: {}",
                    chrono::Utc::now().to_rfc3339()
                ),
                ..Default::default()
            });
            inner_service.call(req).await
        })
    }
}
