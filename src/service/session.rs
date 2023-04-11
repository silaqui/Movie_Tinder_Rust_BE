use std::sync::{Arc, Mutex};

use rocket::State;

use crate::model::common::{SessionStateDTO, VoteDTO};
use crate::model::session::SessionManager;
use crate::model::user_id::UserId;
use crate::service::movie_db;

pub fn start(user_id: &UserId, session_manager: &State<Arc<Mutex<SessionManager>>>) -> SessionStateDTO {
    let mut session_manager = session_manager.inner().lock().unwrap();
    let session_id = session_manager.create_session(user_id);

    let movie = movie_db::get_movie(0);

    SessionStateDTO {
        session_id,
        match_movie: None,
        next_movie: Some(movie),
    }
}

pub fn join(user_id: &UserId, session_id: &String, session_manager: &State<Arc<Mutex<SessionManager>>>) -> SessionStateDTO {
    let mut session_manager = session_manager.inner().lock().unwrap();
    let session_id = session_manager.join(session_id, user_id).expect("Session not found");

    let movie = movie_db::get_movie(0);

    SessionStateDTO {
        session_id: session_id.clone(),
        match_movie: None,
        next_movie: Some(movie),
    }
}

pub fn vote(user_id: &UserId, session_id: &String, vote: VoteDTO, session_manager: &State<Arc<Mutex<SessionManager>>>) -> SessionStateDTO {
    let mut session_manager = session_manager.inner().lock().unwrap();
    let (match_movie, next_movie) = session_manager.vote(session_id, user_id, &vote.movie_id, &vote.result);

    SessionStateDTO {
        session_id: session_id.clone(),
        match_movie: match_movie.0,
        next_movie: next_movie.0,
    }
}

