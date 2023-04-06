use crate::service::model::{Movie, Session, UserId, Vote, VoteResult};
use crate::service::movie_db;


pub fn start(user_id: &UserId) -> Session {

    let session_id =  String::from("1");
    let next = movie_db::get_movie(0);

    Session{
        session_id,
        is_match: false,
        movie : next
    }
}

pub fn join(user_id: &UserId,  session_id: &String) -> Session {

    let session_id =  String::from("1");
    let next = movie_db::get_movie(0);

    Session{
        session_id,
        is_match: false,
        movie : next
    }
}

pub fn vote(user_id: &UserId, session_id: &String, vote: Vote) -> VoteResult {

    let next_id = vote.movie_id.parse::<usize>().unwrap() ;

    let next = movie_db::get_movie(next_id);

    VoteResult{
        is_match: false,
        movie : next
    }
}
