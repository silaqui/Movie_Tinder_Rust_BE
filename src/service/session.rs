use rocket::State;
use crate::service::model::{Movie, Session, Sessions, SessionState, UserId, Vote, VoteResult};
use crate::service::movie_db;

pub fn start(user_id: &UserId, sessions: &State<Sessions>) -> Session {
    let session_id = create_new_session(user_id, sessions);
    let next = get_next_movie(&user_id, &session_id);

    Session {
        session_id,
        is_match: false,
        movie: next,
    }
}

pub fn join(user_id: &UserId, session_id: &String, sessions: &State<Sessions>) -> Session {
    let session_id = join_session(&user_id, &session_id);
    let next = get_next_movie(&user_id, &session_id);

    Session {
        session_id,
        is_match: false,
        movie: next,
    }
}

pub fn vote(user_id: &UserId, session_id: &String, vote: Vote, sessions: &State<Sessions>) -> VoteResult {
    let (is_match, movie) = vote_on_movie(user_id, session_id, vote);

    VoteResult {
        is_match: is_match,
        movie: movie,
    }
}

fn create_new_session(user_id: &UserId, sessions: &State<Sessions>) -> String {
    let s = SessionState{
        movies: vec![],
        votes: vec![],
    };


    String::from("1")
}

fn join_session(user_id: &UserId, session_id: &String) -> String {
    String::from("1")
}

fn get_next_movie(user_id: &UserId, session_id: &String) -> Movie {
    movie_db::get_movie(0)
}

fn vote_on_movie(user_id: &UserId, session_id: &String, vote: Vote) -> (bool, Movie) {
    let next_index = vote.movie_id.parse::<usize>().unwrap();
    let next = movie_db::get_movie(next_index);

    (false, next)
}
