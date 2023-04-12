use crate::service::movie_db;

type MovieId = String;

pub fn generate() -> Vec<MovieId> {
    movie_db::get_all_id()
}