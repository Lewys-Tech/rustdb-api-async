use axum::{extract::Path, routing::get, Router};

async fn get_key(Path(key): Path<String>) -> String {
    key

}

#[tokio::main]
async fn main() {
   let app = Router::new()
        .route("/", get(|| async { "Hello, world!"}))
        .route("/get/{key}", get(get_key));


   

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Listening on 127.0.0.1:8080");
    axum::serve(listener, app).await.unwrap();


}
