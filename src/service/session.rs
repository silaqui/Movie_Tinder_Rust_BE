use std::sync::{Arc, Mutex};

use rocket::State;

use crate::model::common::{NextMovie, Vote};
use crate::model::session::SessionManager;
use crate::model::user_id::UserId;
use crate::service::movie_db;

pub fn start(user_id: &UserId, session_manager: &State<Arc<Mutex<SessionManager>>>) -> NextMovie {
    let mut session_manager = session_manager.inner().lock().unwrap();
    let session_id = session_manager.create_session(user_id);

    let movie = movie_db::get_movie(0);

    NextMovie {
        session_id,
        is_match: false,
        movie,
    }
}

pub fn join(user_id: &UserId, session_id: &String, session_manager: &State<Arc<Mutex<SessionManager>>>) -> NextMovie {
    let mut session_manager = session_manager.inner().lock().unwrap();
    let session_id = session_manager.join(session_id, user_id).expect("Session not found");

    let movie = movie_db::get_movie(0);

    NextMovie {
        session_id: session_id.clone(),
        is_match: false,
        movie,
    }
}

pub fn vote(user_id: &UserId, session_id: &String, vote: Vote, session_manager: &State<Arc<Mutex<SessionManager>>>) -> NextMovie {
    let mut session_manager = session_manager.inner().lock().unwrap();
    let (is_match, movie) = session_manager.vote(session_id, user_id, &vote.movie_id, &vote.result);

    NextMovie {
        session_id: session_id.clone(),
        is_match,
        movie: movie.expect("Should have movie"),
    }
}

