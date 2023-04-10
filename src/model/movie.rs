use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct Movie {
    pub id: String,
    pub title: String,
    pub genres: Vec<String>,
    pub description: String,
    pub poster_url: String,
}

pub struct MovieDetails {
    pub title: String,
    pub position: String,
    pub year: i32,
    pub certificate: String,
    pub runtime: String,
    pub genre: Vec<String>,
    pub description: String,
    pub director: Vec<String>,
    pub stars: Vec<String>,
    pub poster_url: String,
}