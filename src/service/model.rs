use std::sync::atomic::AtomicUsize;

use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Serialize, Deserialize, Clone, Debug)]
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
pub struct Vote {
    pub result: String,
    pub movie_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct VoteResult {
    pub is_match: bool,
    pub movie: Movie,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Session {
    pub session_id: String,
    pub is_match: bool,
    pub movie: Movie,
}

use rocket::request::{self, FromRequest, Request};
use rocket::State;
use crate::service::user;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserId {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<UserId, Self::Error> {
        let cookies = request.cookies();
        let hit_count = request.guard::<&State<HitCount>>().await.unwrap();

        let user_id = user::identify_user(cookies, hit_count);

        request::Outcome::Success(user_id)
    }
}


#[derive(Debug)]
pub struct UserId(pub String);

pub struct Sessions {
    pub sessions: Vec<SessionState>,
}

pub struct SessionState {
    pub movies: Vec<Movie>,
    pub votes: Vec<SessionVote>,
}

pub struct SessionVote {
    pub movie_id: String,
    pub users_id: Vec<UserId>,
    pub vote_watch: Vec<UserId>,
}