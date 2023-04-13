use std::sync::{Arc, Mutex};

use crate::model::common::{SessionStateDTO, VoteDTO};
use crate::model::session_id::SessionId;
use crate::model::user_id::UserId;
use crate::service::movie_db;
use crate::service::session_manager::SessionManager;

pub fn start(user_id: &UserId, session_manager: &Arc<Mutex<SessionManager>>) -> SessionStateDTO {
    let mut session_manager = session_manager.lock().unwrap();
    let session_id = session_manager.create_session(user_id);
    let movie = session_manager.get_first_un_voted(&session_id, &user_id).unwrap();
    let movie = movie.map(|id| movie_db::get_by_id(id)).flatten();

    SessionStateDTO {
        session_id: Some(session_id),
        match_movie: None,
        next_movie: movie,
    }
}

pub fn join(user_id: &UserId, session_id: &SessionId, session_manager: &Arc<Mutex<SessionManager>>) -> SessionStateDTO {
    let mut session_manager = session_manager.lock().unwrap();

    return match session_manager.join(&session_id, user_id) {
        Ok(_) => {
            let movie = session_manager.get_first_un_voted(&session_id, &user_id).unwrap();
            let movie = movie.map(|id| movie_db::get_by_id(id)).flatten();
            SessionStateDTO {
                session_id: Some(session_id.clone()),
                match_movie: None,
                next_movie: movie,
            }
        }
        Err(e) => {
            log::warn!("{}",e);
            SessionStateDTO {
                session_id: None,
                match_movie: None,
                next_movie: None,
            }
        }
    };
}

pub fn vote(user_id: &UserId, session_id: &SessionId, vote: VoteDTO, session_manager: &Arc<Mutex<SessionManager>>) -> SessionStateDTO {
    let mut session_manager = session_manager.lock().unwrap();

    return match session_manager.vote(&session_id, user_id, &vote.movie_id, &vote.result) {
        Ok(_) => {
            let session_match = session_manager.get_session_match(session_id).unwrap();
            let next_movie = session_manager.get_first_un_voted(session_id, user_id).unwrap();

            let match_movie = session_match.map(|id| movie_db::get_by_id(id)).flatten();
            let next_movie = next_movie.map(|id| movie_db::get_by_id(id)).flatten();

            SessionStateDTO {
                session_id: Some(session_id.clone()),
                match_movie,
                next_movie,
            }
        }
        Err(e) => {
            log::warn!("{}",e);
            SessionStateDTO {
                session_id: None,
                match_movie: None,
                next_movie: None,
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use crate::model::user_id::UserId;
    use crate::service::session_manager::SessionManager;

    #[test]
    fn test_start() {
        let manager = Arc::new(Mutex::new(SessionManager::new()));

        let user_id = UserId(String::from("guest_1"));

        let session_state = super::start(&user_id, &manager);
        assert_eq!(session_state.session_id, Some(1));

        let session_state = super::start(&user_id, &manager);
        assert_eq!(session_state.session_id, Some(2));

        let session_state = super::start(&user_id, &manager);
        assert_eq!(session_state.session_id, Some(3));
    }

    #[test]
    fn test_join() {
        let manager = Arc::new(Mutex::new(SessionManager::new()));

        let user_id = UserId(String::from("guest_1"));

        let session_state = super::start(&user_id, &manager);
        assert_eq!(session_state.session_id, Some(1));

        let session_id = session_state.session_id.unwrap();
        let user_id = UserId(String::from("guest_2"));

        let session_state = super::join(&user_id, &session_id, &manager);
        assert_eq!(session_state.session_id, Some(1));
    }

    #[test]
    fn test_join_non_existing_session() {
        let manager = Arc::new(Mutex::new(SessionManager::new()));

        let user_id = UserId(String::from("guest_1"));

        let session_state = super::join(&user_id, &1, &manager);
        assert_eq!(session_state.session_id, None);
    }
}