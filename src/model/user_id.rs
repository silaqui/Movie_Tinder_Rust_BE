use std::sync::atomic::Ordering;

use rocket::http::{Cookie, CookieJar};
use rocket::request::{self, FromRequest, Request};
use rocket::State;

use crate::model::hit_count::HitCount;

pub const USER_ID_FIELD_NAME: &str = "user_id";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserId {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<UserId, Self::Error> {
        let cookies = request.cookies();
        let hit_count = request.guard::<&State<HitCount>>().await.unwrap();

        let user_id = identify_user(cookies, hit_count);

        request::Outcome::Success(user_id)
    }
}

fn identify_user(cookies: &CookieJar<'_>, hit_count: &State<HitCount>) -> UserId {
    let id = match cookies.get(USER_ID_FIELD_NAME) {
        None => {
            let count = hit_count.0.fetch_add(1, Ordering::Relaxed) + 1;
            let id = format!("guest_{}", count);
            log::info!("Missing {}, setting to: {}",USER_ID_FIELD_NAME, id);
            cookies.add(
                Cookie::new(String::from(USER_ID_FIELD_NAME), id.clone())
            );
            id
        }
        Some(id) => {
            log::info!("{} present: {}", USER_ID_FIELD_NAME, id.value());
            id.value().clone().into()
        }
    };

    UserId(id)
}