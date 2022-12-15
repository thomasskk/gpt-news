use axum::{
    extract::{Path, Query},
    routing::get,
    Router,
};

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct News {
    id: String,
    title: String,
    name: String,
}

pub fn create_route() -> Router {
    Router::new()
        .route("/news/:id", get(get_news_by_id))
        .route("/news", get(get_news))
}

async fn get_news() -> Result<Vec<News>> {
    Ok(res)
}

async fn get_news_by_id(Path(id): Path<String>) -> Result<News> {
    Ok(res)
}
