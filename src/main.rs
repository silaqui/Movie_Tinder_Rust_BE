#[macro_use]
extern crate rocket;

use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicUsize;

use rocket::{Build, Rocket};

use crate::model::hit_count::HitCount;
use crate::service::api;
use crate::service::session_manager::SessionManager;

mod service;
mod model;

fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount(api::BASE, api::routes())
        .manage(HitCount(AtomicUsize::new(0)))
        .manage(Arc::new(Mutex::new(SessionManager::new())))
}

#[main]
async fn main() {
    rocket()
        .launch()
        .await
        .expect("Could not start server! Shutting down.");
}
