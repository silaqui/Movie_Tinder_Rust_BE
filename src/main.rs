#[macro_use]
extern crate rocket;

use std::sync::atomic::AtomicUsize;
use model::HitCount;

mod api;
mod model;
mod movie_db;

#[main]
async fn main() {
    rocket::build()
        .mount(api::BASE, api::routes())
        .manage(HitCount(AtomicUsize::new(0)))
        .launch()
        .await
        .expect("Could not start server! Shutting down.");
}
