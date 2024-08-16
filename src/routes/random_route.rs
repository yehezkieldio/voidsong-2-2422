use axum::routing::get;
use axum::Router;

use crate::{handlers::random_image_handler, utils::state::AppState};

pub fn routes() -> Router {
    let state: AppState = AppState::new();

    let animals: Router = Router::new()
        .route("/cat", get(random_image_handler::get_random_cat))
        .route("/dog", get(random_image_handler::get_random_dog))
        .route("/fox", get(random_image_handler::get_random_fox))
        .route("/bunny", get(random_image_handler::get_random_bunny))
        .route("/duck", get(random_image_handler::get_random_duck))
        .with_state(state);

    let router: Router = Router::new().nest("/animal", animals);

    router
}
