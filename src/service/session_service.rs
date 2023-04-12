use std::sync::{Arc, Mutex};

use rocket::State;

use crate::model::common::{SessionStateDTO, VoteDTO};
use crate::model::session_id::SessionId;
use crate::model::user_id::UserId;
use crate::service::movie_db;
use crate::service::session_manager::SessionManager;

pub fn start(user_id: &UserId, session_manager: &State<Arc<Mutex<SessionManager>>>) -> SessionStateDTO {
    let mut session_manager = session_manager.inner().lock().unwrap();
    let session_id = session_manager.create_session(user_id);
    let movie = session_manager.get_first_un_voted(&session_id, &user_id);
    let movie = movie.map(|id| movie_db::get_by_id(id)).flatten();

    SessionStateDTO {
        session_id: Some(session_id),
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
            let movie = movie.map(|id| movie_db::get_by_id(id)).flatten();
            SessionStateDTO {
                session_id: Some(session_id.clone()),
                match_movie: None,
                next_movie: movie,
            }
        }
    };
}

pub fn vote(user_id: &UserId, session_id: &SessionId, vote: VoteDTO, session_manager: &State<Arc<Mutex<SessionManager>>>) -> SessionStateDTO {
    let mut session_manager = session_manager.inner().lock().unwrap();

    return match session_manager.vote(&session_id, user_id, &vote.movie_id, &vote.result) {
        Ok((match_movie, next_movie)) => {
            let match_movie = match_movie.map(|id| movie_db::get_by_id(id)).flatten();
            let next_movie = next_movie.map(|id| movie_db::get_by_id(id)).flatten();

            SessionStateDTO {
                session_id: Some(session_id.clone()),
                match_movie,
                next_movie,
            }
        }
        Err(_) => SessionStateDTO {
            session_id: None,
            match_movie: None,
            next_movie: None,
        }
    };
}

