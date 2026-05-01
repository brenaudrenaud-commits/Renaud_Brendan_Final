mod db;
mod fish;
use axum::{
    Router,
    extract::{Form, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    routing::get,
};
use minijinja::{Environment, context};
use rusqlite::Connection;
use serde::Deserialize;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tower_http::services::ServeDir;

//setup minijinja template enviorment to share data
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
    //load html templates with minijinja templates

    //load base html
    env.add_template("base.html", include_str!("templates/base.html"))
        .expect("failed to load base.html");

    //load home html
    env.add_template("home.html", include_str!("templates/home.html"))
        .expect("failed to load home.html");

    //load error page
    env.add_template("404.html", include_str!("templates/404.html"))
        .expect("failed to load 404.html");

    //open database
    let conn = Connection::open("fish.db").expect("failed to open database");

    //sql statement to create a fish table if I dont already have one
    conn.execute(
        "CREATE TABLE IF NOT EXISTS fish (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            species TEXT NOT NULL,
            length REAL NOT NULL,
            weight REAL NOT NULL
        )",
        [], //no parameters needed
    )
    //if statement fails, stop and print errer message
    .expect("failed to create fish table");

    let state = AppState {
        //share template enviorment
        env: Arc::new(env),

        //share database connection
        //mutex lock to ensure one "thing" is being used at a time
        conn: Arc::new(Mutex::new(conn)), //stores database info
    };

    //define routes i only need a home page, i can do all of my fish
    //information operations there and the status route is from class
    let app = Router::new()
        .route("/", get(home).post(create_fish))
        .route("/status", get(conditional_health_response))
        .nest_service("/static", ServeDir::new("src/static"))
        .fallback(not_found)
        .with_state(state);

    //define local host address
    let addr: SocketAddr = "127.0.0.1:7008".parse().unwrap();
    //bind tokio to local host port address 7008
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind to port");
    println!("Server running on http://{}", addr);

    //start server to "serve" requests to users
    axum::serve(listener, app)
        .await
        .expect("server failed to start");
}

//show homepage with existing fish data!
async fn home(State(state): State<AppState>) -> impl IntoResponse {
    //establish locked connection to ensure acurate information
    let conn = state.conn.lock().unwrap();
    //"Wrap" connection
    let db = db::SqLiteConnection::new(&conn);

    //this displays every fish currently in the database
    match db.get_all_fish() {
        Ok(fish_list) => {
            //this is the page I want the fish displayed on "home.html"
            let tmpl = state.env.get_template("home.html").unwrap();
            //this actually displays/renders the fish
            let rendered = tmpl.render(context! { fish => fish_list }).unwrap();
            Html(rendered).into_response()
        }
        Err(_) => (
            //if all else fails return err from Result Enum
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to load home",
        )
            .into_response(),
    }
}

//data from add-fish "form" is serialized, must "deserialize" to a struct
#[derive(Deserialize)]
struct FishForm {
    name: String,
    species: String,
    length: f64,
    weight: f64,
}

//handler for submitted new fish "form"
async fn create_fish(
    State(state): State<AppState>,
    Form(form): Form<FishForm>,
) -> impl IntoResponse {
    //establish locked connection to ensure acurate information
    let conn = state.conn.lock().unwrap();
    //"Wrap" connection
    let db = db::SqLiteConnection::new(&conn);

    //match to add a new fish into the database
    match db.create_fish(&form.name, &form.species, form.length, form.weight) {
        //if everything is good return ok from Result enum and load the new fish to home page
        Ok(_) => Redirect::to("/").into_response(),
        //otherwise return err
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "failed to create fish").into_response(),
    }
}

//return different status based on condition if everything is
//running correctly when you click the status page it says everything is working
async fn conditional_health_response() -> (StatusCode, &'static str) {
    let health = true;

    if health {
        (StatusCode::OK, "everything is working")
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, "service is down :-(")
    }
}

//custom error 404 page
async fn not_found(State(state): State<AppState>) -> impl IntoResponse {
    let tmpl = state.env.get_template("404.html").unwrap();
    let rendered = tmpl.render(()).unwrap();
    (StatusCode::NOT_FOUND, Html(rendered))
}