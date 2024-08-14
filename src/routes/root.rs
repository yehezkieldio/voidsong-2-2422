use axum::Router;
use tower_http::normalize_path::{NormalizePath, NormalizePathLayer};
use tower_layer::Layer;

use super::random_route;

pub fn routes() -> NormalizePath<Router> {
    let app_router = Router::new().nest("/random", random_route::routes());

    NormalizePathLayer::trim_trailing_slash().layer(app_router)
}