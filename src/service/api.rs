use std::sync::{Arc, Mutex};

use rocket::{Route, State};
use rocket::serde::json::Json;

use crate::model::common::{SessionStateDTO, VoteDTO};
use crate::model::session_id::SessionId;
use crate::model::user_id::UserId;
use crate::service::session_manager::SessionManager;
use crate::service::session_service;

pub const BASE: &str = "/api";

#[get("/start")]
fn start(session_manager: &State<Arc<Mutex<SessionManager>>>, user_id: UserId) -> Json<SessionStateDTO> {
    log::info!("Start session - user: {:?}", user_id);
    Json(session_service::start(&user_id, &session_manager))
}

#[get("/join/<session_id>")]
fn join(session_manager: &State<Arc<Mutex<SessionManager>>>, user_id: UserId, session_id: SessionId) -> Json<SessionStateDTO> {
    log::info!("Join session - user: {:?} - session: {}", user_id, session_id);
    Json(session_service::join(&user_id, &session_id, &session_manager))
}

#[post("/vote/<session_id>", format = "json", data = "<vote>")]
fn vote(session_manager: &State<Arc<Mutex<SessionManager>>>, user_id: UserId, session_id: SessionId, vote: Json<VoteDTO>) -> Json<SessionStateDTO> {
    log::info!("Vote session - user: {:?} - session: {:?} - vote: {:?}", user_id, session_id, vote.0);
    Json(session_service::vote(&user_id, &session_id, vote.0, &session_manager))
}

pub fn routes() -> Vec<Route> {
    routes![start, join, vote ]
}

#[cfg(test)]
mod tests {
    use rocket::{http::Status, local::blocking::Client};

    use crate::{model::common::SessionStateDTO, rocket};
    use crate::model::user_id::USER_ID_FIELD_NAME;

    #[test]
    fn test_start() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        let response = client.get("/api/start").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let user_id = response.cookies().get(USER_ID_FIELD_NAME).unwrap().value();
        assert_eq!(user_id, "guest_1");
        let body = response.into_json::<SessionStateDTO>().unwrap();
        let id: Option<usize> = Some(1);
        assert_eq!(body.session_id, id);
    }
}