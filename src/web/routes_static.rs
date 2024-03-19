use axum::{routing::get_service, Router};

pub fn routes_static() -> Router {
    Router::new()
        .nest_service(
            "/", 
            get_service(
                tower_http::services::ServeDir::new("./")
            )
        )
}

