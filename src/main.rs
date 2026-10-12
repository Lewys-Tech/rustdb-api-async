use std::{collections::HashMap, sync::{Arc, Mutex}};
use axum::{extract::{Path, State}, http::StatusCode,routing::{get, post}, Router};

type Db = Arc<Mutex<HashMap<String, String>>>;

async fn get_key(State(db): State<Db>, Path(key):  Path<String>) -> Result<String, StatusCode> {
    let db = db.lock().unwrap();
    match db.get(&key)  {
        Some(value) => Ok(value.clone()),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn set_key(State(db): State<Db>, body: String) -> Result<&'static str, StatusCode> {
    match body.split_once('=') {
        Some((key, value)) => {
            db.lock().unwrap().insert(key.to_string(), value.to_string());
            Ok("OK")
        }
        None => Err(StatusCode::BAD_REQUEST),
    }
}


#[tokio::main]
async fn main() {
    let db: Db = Arc::new(Mutex::new(HashMap::new()));
    db.lock().unwrap().insert("name".to_string(), "lewis".to_string());


   let app = Router::new()
        .route("/", get(|| async { "Hello, world!"}))
        .route("/get/{key}", get(get_key))
        .route("/set", post(set_key))
        .with_state(db);


   

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Listening on 127.0.0.1:8080");
    axum::serve(listener, app).await.unwrap();


}
