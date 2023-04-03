#[macro_use]
extern crate rocket;

mod api;
mod model;

// #[get("/hello")]
// fn hello() -> String {
//     format!("Hello!", )
// }
//
// #[get("/<name>")]
// fn hello_user(name: String) -> String {
//     format!("Hello {}!", name)
// }
//
//
// #[post("/message", data = "<message>")]
// fn submit(cookies: &CookieJar<'_>, message: &str) -> String {
//     cookies.add(Cookie::new("message", message.to_string()));
//
//     "Check cookie".into()
// }

#[main]
async fn main() {
    rocket::build()
        .mount(api::BASE, api::routes())
        .launch()
        .await
        .expect("Could not start server! Shutting down.");
}
