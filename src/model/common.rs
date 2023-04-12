use rocket::serde::Deserialize;
use rocket::serde::Serialize;

use crate::model::movie::Movie;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct VoteDTO {
    pub result: VoteResult,
    pub movie_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SessionStateDTO {
    pub session_id: Option<usize>,
    pub match_movie: Option<Movie>,
    pub next_movie: Option<Movie>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "rocket::serde")]
pub enum VoteResult {
    WATCH,
    SKIP,
}