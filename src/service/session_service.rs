use std::sync::{Arc, Mutex};

use rocket::State;

use crate::model::common::{SessionStateDTO, VoteDTO};
use crate::model::session::{SessionId, SessionManager};
use crate::model::user_id::UserId;

pub fn start(user_id: &UserId, session_manager: &State<Arc<Mutex<SessionManager>>>) -> SessionStateDTO {
    let mut session_manager = session_manager.inner().lock().unwrap();
    let session_id = session_manager.create_session(user_id);
    let movie = session_manager.get_first_un_voted(&session_id, &user_id);

    SessionStateDTO {
        session_id: Some(session_id.0),
        match_movie: None,
        next_movie: movie,
    }
}

pub fn join(user_id: &UserId, session_id: &SessionId, session_manager: &State<Arc<Mutex<SessionManager>>>) -> SessionStateDTO {
    let mut session_manager = session_manager.inner().lock().unwrap();

    return match session_manager.join(&session_id, user_id) {
        None => SessionStateDTO {
            session_id: None,
            match_movie: None,
            next_movie: None,
        },
        Some(_) => {
            let movie = session_manager.get_first_un_voted(&session_id, &user_id);
            SessionStateDTO {
                session_id: Some(session_id.0.clone()),
                match_movie: None,
                next_movie: movie,
            }
        }
    };
}

pub fn vote(user_id: &UserId, session_id: &SessionId, vote: VoteDTO, session_manager: &State<Arc<Mutex<SessionManager>>>) -> SessionStateDTO {
    let mut session_manager = session_manager.inner().lock().unwrap();

    return match session_manager.vote(&session_id, user_id, &vote.movie_id, &vote.result) {
        Ok((match_movie, next_movie)) => SessionStateDTO {
            session_id: Some(session_id.0.clone()),
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

