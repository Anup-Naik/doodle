use std::env;

use axum::{
    Router,
    extract::{Json, Path, Query},
    routing::get,
};
use dotenv::dotenv;
use mongodb::{
    Client, Collection,
    bson::{Document, doc},
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Pagination {
    page: usize,
    limit: usize,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    connect().await.unwrap();
    app().await;
}

//DB Connection
async fn connect() -> mongodb::error::Result<()> {
    let uri = env::var("MONGO_URL").expect("MONGO_URL Not Found");
    let client = Client::with_uri_str(uri).await?;
    let database = client.database("sample_mflix");
    let my_coll: Collection<Document> = database.collection("movies");
    let my_movie = my_coll.find_one(doc! { "title": "The Perils of Pauline" }).await?;
    match my_movie {
        Some(movie) => println!("Found a movie:\n{:#?}", movie),
        None => println!("No Movies Found"),
    };
    Ok(())
}

//Server
async fn app() {
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
