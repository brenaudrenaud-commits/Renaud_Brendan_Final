use axum::{Router, http::StatusCode, extract::{Path, Query}, response::IntoResponse, Json, routing::{get, post, put}};
use serde::{Serialize, Deserialize};

//tokio -> async multi-threading in rust
//we have been doing standard rust multi-threading: which blocks threads when waiting
//tokio allows threads to do other work while waiting for I/O operation
// macro to set up entry point
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .route("/status", get(conditionalhealthresponse));
        
    //listener waits for requests/people joining the website
    let listener = tokio::net::TcpListener::bind("localhost:7008")
        .await
        .expect("failed ot bind to port");

    //"serve" requests to users
    axum::serve(listener, app)
        .await
        .expect("server failed to start");
}

#[derive(Serialize)]
struct User{
    id: u64,
    name: String,
    age: u32,
}

#[derive(Serialize)]
struct Post {
    user_id: u64,
    post_id: u64,
}


//ensure this thread doesnt have a lifetime 
//handler asynchronous function that returns anything implementing "IntoResponse" trait
async fn home() -> &'static str {
    "--WELCOME TO--\n FISH TRACKER"
}

//return different status based on condition
async fn conditionalhealthresponse() -> (StatusCode, &'static str) {
    let health = true;
    if health {
        (StatusCode::OK, "everything is working")
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, "service is down :-(")
    }
}
