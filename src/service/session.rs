use std::sync::{Arc, Mutex};

use rocket::State;

use crate::model::common::{SessionStateDTO, VoteDTO};
use crate::model::session::{SessionId, SessionManager};
use crate::model::user_id::UserId;
use crate::service::movie_db;

pub fn start(user_id: &UserId, session_manager: &State<Arc<Mutex<SessionManager>>>) -> SessionStateDTO {
    let mut session_manager = session_manager.inner().lock().unwrap();
    let session_id = session_manager.create_session(user_id);

    let movie = movie_db::get_movie(0);

    SessionStateDTO {
        session_id: Some(session_id.0),
        match_movie: None,
        next_movie: Some(movie),
    }
}

pub fn join(user_id: &UserId, session_id: &usize, session_manager: &State<Arc<Mutex<SessionManager>>>) -> SessionStateDTO {
    let mut session_manager = session_manager.inner().lock().unwrap();

    return match session_manager.join(&SessionId(session_id.clone()), user_id) {
        None => SessionStateDTO {
            session_id: None,
            match_movie: None,
            next_movie: None,
        },
        Some(_) => {
            let movie = movie_db::get_movie(0);
            SessionStateDTO {
                session_id: Some(session_id.0),
                match_movie: None,
                next_movie: Some(movie),
            }
        }
    };
}

pub fn vote(user_id: &UserId, session_id: &usize, vote: VoteDTO, session_manager: &State<Arc<Mutex<SessionManager>>>) -> SessionStateDTO {
    let mut session_manager = session_manager.inner().lock().unwrap();

    return match session_manager.vote(&SessionId(session_id.clone()), user_id, &vote.movie_id, &vote.result) {
        Ok((match_movie, next_movie)) => SessionStateDTO {
            session_id: Some(*session_id),
            match_movie: match_movie.0,
            next_movie: next_movie.0,
        },
        Err(_) => SessionStateDTO {
            session_id: None,
            match_movie: None,
            next_movie: None,
        }
    };
}

