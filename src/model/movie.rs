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
