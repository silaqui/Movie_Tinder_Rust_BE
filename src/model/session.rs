use std::collections::HashMap;

use rocket::serde::Deserialize;
use rocket::serde::Serialize;

use crate::model::movie::Movie;
use crate::model::session::VoteResult::WATCH;
use crate::model::user_id::UserId;
use crate::service::movie_db;

pub struct SessionManager {
    sessions: Vec<SessionState>,
}

#[derive(Debug)]
pub struct SessionState {
    pub id: String,
    pub users: Vec<UserId>,
    pub votes: Vec<MovieVote>,
    pub session_result: Option<Movie>,
}

#[derive(Debug, PartialEq)]
pub struct MovieVote {
    pub movie: Movie,
    pub votes: HashMap<UserId, VoteResult>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "rocket::serde")]
pub enum VoteResult {
    WATCH,
    SKIP,
}

impl SessionManager {
    pub fn new() -> Self {
        SessionManager {
            sessions: Vec::new(),
        }
    }

    pub fn create_session(&mut self, user_id: &UserId) -> String {
        let session_id = String::from("1");
        let new_session = SessionState {
            id: session_id.clone(),
            users: Vec::from([user_id.clone()]),
            votes: movie_db::get_movies().iter().map(
                |m| MovieVote {
                    movie: m.clone(),
                    votes: HashMap::new(),
                }
            ).collect(),
            session_result: None,
        };
        log::info!("New session {:?}", new_session);
        self.sessions.push(new_session);
        session_id
    }


    pub fn join(&mut self, session_id: &String, user_id: &UserId) -> Option<&String> {
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

    pub fn vote(&mut self, session_id: &String, user_id: &UserId, movie_id: &String, vote_result: &VoteResult) -> (bool, Option<Movie>) {
        if let Some(session) = self.sessions.iter_mut().find(|s| &s.id == session_id) {
            log::info!("Found session");
            log::info!("Users: {:?}" , session.users);
            let movie_vote_index = session.votes.iter().position(|mv| mv.movie.id == *movie_id);

            if let Some(movie_vote) = session.votes.iter_mut().find(|mv| {
                log::info!("Check {} with {}",mv.movie.id, *movie_id );
                mv.movie.id == *movie_id
            }) {
                log::info!("Found movie");
                if !movie_vote.votes.contains_key(user_id) {
                    movie_vote.votes.insert(user_id.clone(), vote_result.clone());
                }

                log::info!("Votes : {:?}", movie_vote.votes);

                let all_users_voted = movie_vote.votes.len() == session.users.len();
                let is_match = movie_vote.votes.iter().all(|v| { *v.1 == WATCH });

                log::info!("all_users_voted {} | is_match {}",all_users_voted, is_match);

                return if all_users_voted && is_match {
                    log::info!("Vote result: true / {:?} " , movie_vote.movie);
                    (true, Some(movie_vote.movie.clone()))
                } else {
                    let current_index = movie_vote_index.unwrap();
                    let next_index = current_index + 1;
                    return if next_index >= session.votes.len() {
                        log::info!("Vote result: false / None - No more movies");
                        (false, None)
                    } else {
                        let next_movie = session.votes[next_index].movie.clone();
                        log::info!("Vote result: false / {:?} " , next_movie);
                        (false, Some(next_movie))
                    }
                }
            } else {
                panic!("Invalid movie id.");
            }
        } else {
            panic!("Invalid session id.");
        }
    }
}