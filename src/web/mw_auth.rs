use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::RequestPartsExt;
use async_trait::async_trait;
use axum::{body::Body, http::Request, middleware::Next, response::Response};
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};
use crate::model::ModelController;
use crate::{Error, Result};
use crate::ctx::Ctx;

use crate::web;


pub async fn mw_req_auth(
    // cookies: Cookies,
    ctx: Result<Ctx>,
    req: Request<Body>,
    next: Next
) -> Result<Response> {
    println!("->> {:<12} - mw_auth", "MIDDLEWARE");

    // let (user_id, exp, sig) = cookies
    //     .get(web::AUTH_TOKEN)
    //     .map(|c| c.value().to_string())
    //     .ok_or(Error::AuthFailNoAuthToken)
    //     .and_then(parse_token)?;
    ctx?;

    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolver(
    _mc: State<ModelController>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next
) -> Result<Response> {
    println!("->> {:<12} - mw_ctx_resolver", "MIDDLEWARE");

    let auth_token = cookies.get(web::AUTH_TOKEN).map(|c| c.value().to_string());

    let result_ctx = match auth_token
        .ok_or(Error::AuthFailNoAuthToken)
        .and_then(parse_token) {
            Ok((user_id, _exp, _sig)) => {
                Ok(Ctx::new(user_id))
            },
            Err(e) => Err(e)
        };

    if result_ctx.is_err() && !matches!(result_ctx, Err(Error::AuthFailNoAuthToken)) {
        cookies.remove(Cookie::from(web::AUTH_TOKEN));
    }

    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}

// extractor for ctx
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:<12} - Ctx", "EXTRACTOR");

        // let cookies = parts.extract::<Cookies>().await.unwrap();

        // let (user_id, exp, sig) = cookies
        //     .get(web::AUTH_TOKEN) 
        //     .map(|c| c.value().to_string())
        //     .ok_or(Error::AuthFailWrongTokenFormat)
        //     .and_then(parse_token)?;

        // Ok(Ctx::new(user_id))
        parts
            .extensions
            .get::<Result<Ctx>>()
            .ok_or(Error::AuthFailCtxNotInReqExt)?
            .clone()
    }
}

fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sig) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#,
        &token
    ).unwrap();
    let user_id: u64 = user_id.parse().unwrap();
    Ok((user_id, exp.to_string(), sig.to_string()))
}