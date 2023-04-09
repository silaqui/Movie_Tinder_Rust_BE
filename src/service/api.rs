use rocket::{Route, State};
use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::Json;

use crate::service::{movie_db, session, user};
use crate::service::model::{HitCount, Movie, Session, Sessions, UserId, Vote, VoteResult};

pub const BASE: &str = "/api";

#[get("/start")]
fn start(user_id: UserId, sessions: &State<Sessions>) -> Json<Session> {
    log::info!("Start session - user: {:?}", user_id);

    Json(session::start(&user_id, &sessions))
}

#[get("/join/<session_id>")]
fn join(user_id: UserId, sessions: &State<Sessions>, session_id: String) -> Json<Session> {
    log::info!("Join session - user: {:?} - session: {}", user_id, session_id);

    Json(session::join(&user_id, &session_id, &sessions))
}

#[post("/vote/<session_id>", format = "json", data = "<vote>")]
fn vote(user_id: UserId, sessions: &State<Sessions>, session_id: String, vote: Json<Vote>) -> Json<VoteResult> {
    let vote: Vote = vote.0;

    log::info!("Vote session - user: {:?} - session: {:?} - vote: {:?}", user_id, session_id, vote);

    Json(session::vote(&user_id, &session_id, vote, &sessions))
}

#[get("/movie")]
fn movies() -> Json<Movie> {
    Json(movie_db::get_movie(4))
}

#[get("/clean")]
fn clear(cookies: &CookieJar<'_>) {
    cookies.remove(Cookie::named("session_id"));
}

pub fn routes() -> Vec<Route> {
    routes![start, join, vote, movies ]
}