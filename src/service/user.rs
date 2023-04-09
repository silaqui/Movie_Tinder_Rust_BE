use std::sync::atomic::Ordering;

use rocket::http::{Cookie, CookieJar};
use rocket::State;

use crate::model::common::HitCount;
use crate::model::user_id::UserId;

pub fn identify_user(cookies: &CookieJar<'_>, hit_count: &State<HitCount>) -> UserId {
    let name = "user_id";

    let id = match cookies.get(name) {
        None => {
            let count = hit_count.0.fetch_add(1, Ordering::Relaxed) + 1;
            let id = format!("guest_{}", count);
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

    UserId(id)
}
