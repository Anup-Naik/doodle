use axum::{
    Router,
    extract::{Json, Path, Query},
    routing::get,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Pagination {
    page: usize,
    limit: usize,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        .route("/{name}", get(name_get).post(name_post));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

//Handlers
async fn hello() -> &'static str {
    "Greeter\n"
}

async fn name_get(Path(name): Path<String>, pagination: Query<Pagination>) -> String {
    let Query(p) = pagination;
    println!("Pagination => page{},perpage{}", p.page, p.limit);
    format!("Hello {}\n", name)
}

async fn name_post(Json(payload): Json<serde_json::Value>) -> String {
    println!("{:?}", payload.to_string());
    payload.to_string()
}
