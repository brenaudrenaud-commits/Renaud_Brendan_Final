use serde::{Deserialize, Serialize};

//struct fish to store information about a fish that already exists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fish {
    pub id: i64,
    pub name: String,
    pub species: String,
    pub length: f64,
    pub weight: f64,
    pub day_caught: String,
    pub user_id: i64,
}

//newfish has all of the same fields except for id, so when we "make"
// a new fish all of these rows are inserted and a uniqe id is generated to store as fish
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewFish {
    pub name: String,
    pub species: String,
    pub length: f64,
    pub weight: f64,
    pub day_caught: String,
    pub user_id: i64,
}

//struct to store username and id for anyone on the website 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
}