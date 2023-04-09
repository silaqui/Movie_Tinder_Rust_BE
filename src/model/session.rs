use crate::model::movie::Movie;
use crate::model::user_id::UserId;

pub struct SessionManager {
    sessions: Vec<SessionState>,
}

pub struct SessionState {
    pub id: String,
    pub users: Vec<UserId>,
    pub movies: Vec<Movie>,
    pub votes: Vec<SessionVote>,
    pub session_result: Option<Movie>,
}

pub struct SessionVote {
    pub movie_id: String,
    pub users_id: Vec<UserId>,
    pub vote_watch: Vec<UserId>,
}

impl SessionManager {
    pub fn new() -> Self {
        SessionManager {
            sessions: Vec::new(),
        }
    }

    pub fn create_session(&mut self) -> String {
        let session_id = String::from("1");
        let new_session = SessionState {
            id: session_id.clone(),
            users: Vec::new(),
            movies: Vec::new(),
            votes: Vec::new(),
            session_result: None,
        };
        self.sessions.push(new_session);
        session_id
    }
}