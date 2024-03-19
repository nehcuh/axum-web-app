use serde::Deserialize;
use axum::{routing::post, Json, Router};
use tower_cookies::{Cookies, Cookie};
use serde_json::{json, Value};
use crate::{web, Error, Result};

pub fn routes() -> Router {
    Router::new()
        .route(
        "/api/login",
        post(api_login)
    )
}

#[derive(Deserialize)]
pub struct LoginPayload {
    username: String,
    password: String
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    if payload.username != "demo" || payload.password != "welcome" {
        return Err(Error::LoginFail)
    } 

    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sig"));

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}