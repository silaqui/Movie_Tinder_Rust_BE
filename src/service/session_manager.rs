use std::collections::HashMap;

use crate::model::common::VoteResult;
use crate::model::common::VoteResult::WATCH;
use crate::model::session_id::SessionId;
use crate::model::user_id::UserId;
use crate::service::movie_list_generator;

pub struct SessionManager {
    sessions: Vec<SessionState>,
}

#[derive(Debug)]
struct SessionState {
    id: SessionId,
    users: Vec<UserId>,
    votes: Vec<MovieVote>,
}

#[derive(Debug, PartialEq)]
struct MovieVote {
    movie_id: String,
    votes: HashMap<UserId, VoteResult>,
}

type MovieId = String;

impl SessionState {
    pub fn new(id: SessionId, movies_ids: Vec<MovieId>) -> Self {
        SessionState {
            id,
            users: Vec::new(),
            votes: movies_ids.iter().map(
                |id| MovieVote {
                    movie_id: id.clone(),
                    votes: HashMap::new(),
                }
            ).collect(),
        }
    }

    pub fn add_user(&mut self, user_id: &UserId) {
        if !self.users.contains(user_id) {
            let _ = &self.users.push(user_id.clone());
            log::info!("Joined session {:?}", self);
        } else {
            log::info!("Already in session {:?}", self);
        }
    }
}

impl SessionManager {
    pub fn new() -> Self {
        SessionManager {
            sessions: Vec::new(),
        }
    }

    pub fn create_session(&mut self, user_id: &UserId) -> SessionId {
        let session_id = self
            .sessions
            .last()
            .map(|s| s.id + 1)
            .unwrap_or(1);

        let mut new_session = SessionState::new(session_id, movie_list_generator::generate());
        new_session.add_user(user_id);
        log::info!("New session {:?}", new_session);
        self.sessions.push(new_session);
        session_id
    }


    pub fn join(&mut self, session_id: &SessionId, user_id: &UserId) -> Option<&SessionId> {
        let s = self.sessions.iter_mut().find(|v| v.id == *session_id);

        return match s {
            None => { None }
            Some(s) => {
                s.add_user(user_id);
                Some(&s.id)
            }
        };
    }

    pub fn vote(&mut self, session_id: &SessionId, user_id: &UserId, movie_id: &MovieId, vote_result: &VoteResult) -> Result<(), &str> {
        return if let Some(session) = self
            .sessions
            .iter_mut()
            .find(|s| &s.id == session_id) {
            if let Some(movie_vote) = session
                .votes
                .iter_mut()
                .find(|mv| { mv.movie_id == *movie_id }) {
                if !movie_vote.votes.contains_key(user_id) {
                    movie_vote.votes.insert(user_id.clone(), vote_result.clone());
                }

                log::info!("Votes : {:?}", movie_vote.votes);

                Ok(())
            } else {
                Err("Invalid movie id.")
            }
        } else {
            Err("Invalid session id.")
        };
    }

    pub fn get_session_match(&self, session_id: &SessionId) -> Option<MovieId> {
        if let Some(session) = self.sessions.iter().find(|s| &s.id == session_id) {
            return session.votes.iter().find(|movie| {
                let all_users_voted = movie.votes.len() == session.users.len();
                let is_match = movie.votes.iter().all(|v| { *v.1 == WATCH });
                all_users_voted && is_match
            }).map(|mv| mv.movie_id.clone());
        }
        None
    }

    pub fn get_first_un_voted(&self, session_id: &SessionId, user_id: &UserId) -> Option<MovieId> {
        if let Some(session) = self.sessions.iter().find(|s| &s.id == session_id) {
            if let Some(movie_vote) = session
                .votes
                .iter()
                .find(|mv| { !mv.votes.contains_key(user_id) }) {
                return Some(movie_vote.movie_id.clone());
            }
        }
        None
    }
}