use model::ModelController;
/// 1. routes_hello
/// - basic anonymous routes_hello
/// - routes handler
/// - path and query extraction
/// - quick_dev
/// 2. routes_static
/// - tower-http importation
/// - fs service enabled
/// 3. cookies 
/// - tower-cookies enabled
/// - main_response_mapper
/// 4. crud implementation
/// - model layer
/// - crud handlers
/// - basic cookies validation
/// 5. ctx implementation
/// - user defined parameter
/// - ctx extraction
/// - regex support
use tokio::net::TcpListener;
use axum::{middleware, response::Response, routing::get_service, Router};
use tower_cookies::CookieManagerLayer;
use web::mw_auth;

mod web;
mod error;
mod model;
mod ctx;

pub use crate::error::{Result, Error};

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;
    let routes_tickets = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_req_auth));
    let routes_all = Router::new()
        .merge(web::routes_hello::routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_tickets)
        .layer(middleware::map_response(main_resp_mapper))
        .layer(middleware::from_fn_with_state(mc.clone(), web::mw_auth::mw_ctx_resolver))
        .layer(CookieManagerLayer::new())
        // .fallback_service(web::routes_static::routes_static()); error occured but don't know why
        .fallback_service(routes_static());

    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    axum::serve(listener, routes_all.into_make_service()).await.unwrap();
    Ok(())
}

async fn main_resp_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "MIDDLEWARE");
    println!();
    res
}

fn routes_static() -> Router {
    Router::new()
    .nest_service(
        "/", 
        get_service(
            tower_http::services::ServeDir::new("./")
        )
    )
}
