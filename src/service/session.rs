use rocket::State;

use crate::model::common::{NextMovie, Vote};
use crate::model::session::SessionManager;
use crate::model::user_id::UserId;

pub fn start(user_id: &UserId, session_manager: &State<SessionManager>) -> NextMovie {
    // session_manager.create_session();

    todo!()
}

pub fn join(user_id: &UserId, session_id: &String, sessions: &State<SessionManager>) -> NextMovie {
    todo!()
}

pub fn vote(user_id: &UserId, session_id: &String, vote: Vote, sessions: &State<SessionManager>) -> NextMovie {
    todo!()
}
