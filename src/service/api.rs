use rocket::{Route, State};
use rocket::serde::json::Json;

use crate::model::common::{NextMovie, Vote};
use crate::model::session::SessionManager;
use crate::model::user_id::UserId;
use crate::service::session;

pub const BASE: &str = "/api";

#[get("/start")]
fn start(user_id: UserId, session_manager: &State<SessionManager>) -> Json<NextMovie> {
    log::info!("Start session - user: {:?}", user_id);
    Json(session::start(&user_id, &session_manager))
}

#[get("/join/<session_id>")]
fn join(user_id: UserId, session_manager: &State<SessionManager>, session_id: String) -> Json<NextMovie> {
    log::info!("Join session - user: {:?} - session: {}", user_id, session_id);
    Json(session::join(&user_id, &session_id, &session_manager))
}

#[post("/vote/<session_id>", format = "json", data = "<vote>")]
fn vote(user_id: UserId, session_manager: &State<SessionManager>, session_id: String, vote: Json<Vote>) -> Json<NextMovie> {
    log::info!("Vote session - user: {:?} - session: {:?} - vote: {:?}", user_id, session_id, vote.0);
    Json(session::vote(&user_id, &session_id, vote.0, &session_manager))
}

pub fn routes() -> Vec<Route> {
    routes![start, join, vote ]
}