mod service;

#[macro_use]
extern crate rocket;

use std::sync::atomic::AtomicUsize;
use crate::service::api;
use crate::service::model::HitCount;

#[main]
async fn main() {
    rocket::build()
        .mount(api::BASE, api::routes())
        .manage(HitCount(AtomicUsize::new(0)))
        .launch()
        .await
        .expect("Could not start server! Shutting down.");
}
