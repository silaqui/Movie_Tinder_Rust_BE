use std::sync::atomic::AtomicUsize;

use rocket::serde::Deserialize;
use rocket::serde::Serialize;

use crate::model::movie::Movie;
use crate::model::session::VoteResult;

pub struct HitCount(pub AtomicUsize);

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Vote {
    pub result: VoteResult,
    pub movie_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NextMovie {
    pub session_id: String,
    pub is_match: bool,
    pub movie: Movie,
}

