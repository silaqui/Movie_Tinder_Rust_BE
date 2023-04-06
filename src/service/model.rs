use std::sync::atomic::AtomicUsize;
use rocket::serde::Serialize;
use rocket::serde::Deserialize;

#[derive(Serialize, Clone, Debug)]
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

pub struct HitCount(pub AtomicUsize);

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Vote{
    pub result : String,
    pub movie_id : String,
}

pub struct VoteResult{
    pub is_match: bool,
    pub movie: Movie,
}

#[derive(Debug)]
pub struct Session {
    pub session_id : String,
    pub is_match: bool,
    pub movie: Movie,
}

#[derive(Debug)]
pub struct UserId(pub String);
