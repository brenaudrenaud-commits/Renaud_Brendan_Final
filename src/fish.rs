use serde::{Deserialize, Serialize};

//struct fish to store information about a fish that already exists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fish {
    pub id: i64,
    pub name: String,
    pub species: String,
    pub length: f64,
    pub weight: f64,
}
