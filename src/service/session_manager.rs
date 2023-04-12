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
    session_match: Option<String>,
}

#[derive(Debug, PartialEq)]
struct MovieVote {
    movie_id: String,
    votes: HashMap<UserId, VoteResult>,
}

type MovieId = String;
type MatchMovie = Option<String>;
type NextMovie = Option<String>;

impl SessionManager {
    pub fn new() -> Self {
        SessionManager {
            sessions: Vec::new(),
        }
    }

    pub fn create_session(&mut self, user_id: &UserId) -> SessionId {
        let id = self
            .sessions
            .last()
            .map(|s| s.id)
            .map(|n| (n + 1))
            .unwrap_or(1);

        let session_id = id;
        let new_session = SessionState {
            id: session_id.clone(),
            users: Vec::from([user_id.clone()]),
            votes: movie_list_generator::generate().iter().map(
                |id| MovieVote {
                    movie_id: id.clone(),
                    votes: HashMap::new(),
                }
            ).collect(),
            session_match: None,
        };
        log::info!("New session {:?}", new_session);
        self.sessions.push(new_session);
        session_id
    }


    pub fn join(&mut self, session_id: &SessionId, user_id: &UserId) -> Option<&SessionId> {
        let s = self.sessions.iter_mut().find(|v| v.id == *session_id);

        return match s {
            None => { None }
            Some(_) => {
                let u = s.unwrap();
                if !u.users.contains(user_id) {
                    let _ = &u.users.push(user_id.clone());
                    log::info!("Joined session {:?}", u);
                } else {
                    log::info!("Already in session {:?}", u);
                }
                Some(&u.id)
            }
        };
    }

    pub fn vote(&mut self, session_id: &SessionId, user_id: &UserId, movie_id: &MovieId, vote_result: &VoteResult) -> Result<(MatchMovie, NextMovie), &str> {
        return if let Some(session) = self
            .sessions
            .iter_mut()
            .find(|s| &s.id == session_id) {
            let movie_vote_index = session
                .votes
                .iter()
                .position(|mv| mv.movie_id == *movie_id);

            if let Some(movie_vote) = session
                .votes
                .iter_mut()
                .find(|mv| { mv.movie_id == *movie_id }) {
                if !movie_vote.votes.contains_key(user_id) {
                    movie_vote.votes.insert(user_id.clone(), vote_result.clone());
                }

                log::info!("Votes : {:?}", movie_vote.votes);

                if session.session_match == None {
                    let all_users_voted = movie_vote.votes.len() == session.users.len();
                    let is_match = movie_vote.votes.iter().all(|v| { *v.1 == WATCH });

                    if all_users_voted && is_match {
                        log::info!("Vote | Match | {:?} " , movie_vote.movie_id);
                        session.session_match = Some(movie_vote.movie_id.clone());
                    };
                }

                let next_movie = {
                    let current_index = movie_vote_index.unwrap();
                    let next_index = current_index + 1;

                    if next_index >= session.votes.len() {
                        log::info!("Vote | Next | None - No more movies");
                        None
                    } else {
                        let next_movie = session.votes[next_index].movie_id.clone();
                        log::info!("Vote | Next | {:?} " , next_movie);
                        Some(next_movie)
                    }
                };

                Ok((session.session_match.clone(), next_movie))
            } else {
                Err("Invalid movie id.")
            }
        } else {
            Err("Invalid session id.")
        };
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