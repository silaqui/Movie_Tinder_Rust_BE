#[macro_use]
extern crate rocket;

use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicUsize;

use crate::model::common::HitCount;
use crate::model::session::SessionManager;
use crate::service::api;

mod service;
mod model;

#[main]
async fn main() {
    rocket::build()
        .mount(api::BASE, api::routes())
        .manage(HitCount(AtomicUsize::new(0)))
        .manage(Arc::new(Mutex::new(SessionManager::new())))
        .launch()
        .await
        .expect("Could not start server! Shutting down.");
}
