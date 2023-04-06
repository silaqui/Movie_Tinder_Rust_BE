use rocket::{Route, State};
use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::Json;
use crate::service::model::{HitCount, Movie, Session, Vote, VoteResult};
use crate::service::{movie_db, session, user};

pub const BASE: &str = "/api";

#[get("/start")]
fn start(cookies: &CookieJar<'_>, hit_count: &State<HitCount>) -> Json<Session> {
    let user_id = user::identify_user(cookies, hit_count);

    log::info!("Start session - user: {:?}", user_id);

    Json(session::start(&user_id))
}

#[get("/join/<session_id>")]
fn join(cookies: &CookieJar<'_>, hit_count: &State<HitCount>, session_id: String) -> Json<Session> {

    let user_id = user::identify_user(cookies, hit_count);

    log::info!("Join session - user: {:?} - session: {}", user_id, session_id);

    Json(session::join(&user_id, &session_id))
}

#[post("/vote/<session_id>" , format = "json", data = "<vote>" )]
fn vote(cookies: &CookieJar<'_>, hit_count: &State<HitCount>, session_id: String, vote: Json<Vote>) -> Json<VoteResult> {

    let user_id = user::identify_user(cookies, hit_count);

    let vote : Vote = vote.0;

    log::info!("Join session - user: {:?} - session: {:?} - vote: {:?}", user_id, session_id, vote);

    Json(session::vote(&user_id, &session_id, vote))
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