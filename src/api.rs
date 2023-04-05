use std::sync::atomic::Ordering;

use rocket::{Route, State};
use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::Json;

use model::Movie;
use crate::{HitCount, model, movie_db};

pub const BASE: &str = "/api";

fn generate_user_id(i: usize) -> String {
    format!("guest_{}", i)
}

fn generate_session_id(_: usize) -> String {
    format!("session_1")
}

fn get_or_set_user_id(cookies: &CookieJar<'_>, name: &str, generate_fn: fn(usize) -> String, i: usize) -> String {
    return match cookies.get(name) {
        None => {
            let id = generate_fn(i);
            log::info!("Missing {}, setting to: {}",name, id);
            cookies.add(
                Cookie::new(String::from(name), id.clone())
            );
            id
        }
        Some(id) => {
            log::info!("{} present: {}", name, id.value());
            id.value().clone().into()
        }
    };
}

#[get("/start")]
fn start(cookies: &CookieJar<'_>, hit_count: &State<HitCount>) -> Json<Movie> {
    let count = hit_count.0.fetch_add(1, Ordering::Relaxed) + 1;

    get_or_set_user_id(&cookies, "user_id", generate_user_id, count);
    get_or_set_user_id(&cookies, "session_id", generate_session_id, count);

    Json(movie_db::get_movie(count))
}

#[get("/movie")]
fn movies(cookies: &CookieJar<'_>, hit_count: &State<HitCount>) -> Json<Movie> {
    let count = hit_count.0.fetch_add(1, Ordering::Relaxed) + 1;

    get_or_set_user_id(&cookies, "user_id", generate_user_id, count);
    get_or_set_user_id(&cookies, "session_id", generate_session_id, count);

    Json(movie_db::get_movie(count))
}

#[get("/clean")]
fn clear(cookies: &CookieJar<'_>) {
    cookies.remove(Cookie::named("session_id"));
}

pub fn routes() -> Vec<Route> {
    routes![start, movies, clear]
}