use crate::service::model::{Movie, Session, UserId, Vote, VoteResult};
use crate::service::movie_db;


pub fn start(user_id: UserId) -> Session {


    Session{
        session_id : String::from("1"),
        is_match: false,
        movie : movie_db::get_movie(1)
    }
}

pub fn join(user_id: UserId, session_id: String) -> Session {


    Session{
        session_id : String::from("1"),
        is_match: false,
        movie : movie_db::get_movie(1)
    }
}

pub fn vote(user_id: UserId, session_id: String, vote: Vote) -> VoteResult {


    VoteResult{
        is_match: false,
        movie : movie_db::get_movie(4)
    }
}

