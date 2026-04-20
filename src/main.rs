mod db;
mod fish;
use axum::{extract::State,http::StatusCode,response::{Html, IntoResponse},routing::get,Router};
use minijinja::{context, Environment};
use std::{net::SocketAddr, sync::{Arc, Mutex}};
use rusqlite::Connection;
use serde::Deserialize;
use axum::Form;
use axum::response::Redirect;
use tower_http::services::ServeDir;

//setup minijinja template enviorment
#[derive(Clone)]
struct AppState {
    env: Arc<Environment<'static>>,
    conn: Arc<Mutex<Connection>>,
}

//tokio -> async multi-threading in rust
//we have been doing standard rust multi-threading: which blocks threads when waiting
//tokio allows threads to do other work while waiting for I/O operation
// macro to set up entry point
#[tokio::main]
async fn main() {
    let mut env = Environment::new();

    //minijinja with .expect() instead of .unwrap() for better err handling
    env.add_template("base.html", include_str!("templates/base.html"))
        .expect("failed to load base.html");

    env.add_template("home.html", include_str!("templates/home.html"))
        .expect("failed to load home.html");

    env.add_template("404.html", include_str!("templates/404.html"))
        .expect("failed to load 404.html");
    
    let conn = Connection::open("fish.db").expect("failed to open database");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS fish (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            species TEXT NOT NULL,
            length REAL NOT NULL,
            weight REAL NOT NULL
        )",
        [],
    )
    .expect("failed to create fish table");

    let state = AppState {
        env: Arc::new(env),
        conn: Arc::new(Mutex::new(conn)),//stores fish
    };

    let app = Router::new()
        .route("/", get(home).post(create_fish))
        .route("/status", get(conditional_health_response))
        .nest_service("/static", ServeDir::new("src/static"))
        .fallback(not_found)
        .with_state(state);

    let addr: SocketAddr = "127.0.0.1:7008".parse().unwrap();
    //bind tokio to local host port 7008
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind to port");
    println!("Server running on http://{}", addr);

    //"serve" requests to users
    axum::serve(listener, app)
        .await
        .expect("server failed to start");
}

//handler asynchronous function that returns anything implementing "IntoResponse" trait
async fn home(State(state): State<AppState>) -> impl IntoResponse {
    let conn = state.conn.lock().unwrap();
    let db = db::SqlLiteConnection::new(&conn);

    match db.get_all_fish() {
        Ok(fish_list) => {
            let tmpl = state.env.get_template("home.html").unwrap();
            let rendered = tmpl.render(context! { fish => fish_list }).unwrap();
            Html(rendered).into_response()
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to load home"
        ).into_response(),
    }
}

#[derive(Deserialize)]
struct FishForm {
    name: String,
    species: String,
    length: f64,
    weight: f64,
}

async fn create_fish(
    State(state): State<AppState>,
    Form(form): Form<FishForm>,
) -> impl IntoResponse {
    let conn = state.conn.lock().unwrap();
    let db = db::SqlLiteConnection::new(&conn);

    match db.create_fish(&form.name, &form.species, form.length, form.weight) {
        Ok(_) => Redirect::to("/").into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to create fish",
        )
            .into_response(),
    }
}

//return different status based on condition
async fn conditional_health_response() -> (StatusCode, &'static str) {
    let health = true;

    if health {
        (StatusCode::OK, "everything is working")
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, "service is down :-(")
    }
}

//error 404 page
async fn not_found(State(state): State<AppState>) -> impl IntoResponse {
    let tmpl = state.env.get_template("404.html").unwrap();
    let rendered = tmpl.render(()).unwrap();
    (StatusCode::NOT_FOUND, Html(rendered))
}
