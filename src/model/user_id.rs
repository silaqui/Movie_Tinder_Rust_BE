use rocket::request::{self, FromRequest, Request};
use rocket::State;

use crate::model::common::HitCount;
use crate::service::user;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserId {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<UserId, Self::Error> {
        let cookies = request.cookies();
        let hit_count = request.guard::<&State<HitCount>>().await.unwrap();

        let user_id = user::identify_user(cookies, hit_count);

        request::Outcome::Success(user_id)
    }
}
