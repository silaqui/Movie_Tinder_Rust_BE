use rocket::Route;
use rocket::serde::json::Json;

use model::Movie;

use crate::model;

pub const BASE: &str = "/api";

#[get("/start")]
fn start() {

}

#[get("/movie")]
fn movies() -> Json<Movie> {
    let _list = vec![
        Movie {
            id: "1".into(),
            title: "Inception".into(),
            genres: vec!["Action".into(), "Adventure".into(), "Sci-Fi".into()],
            description: "A thief who steals corporate secrets through the use of dream-sharing technology is given the inverse task of planting an idea into the mind of a C.E.O., but his tragic past may doom the project and his team to disaster.".into(),
            poster_url: "https://cdn.shopify.com/s/files/1/0037/8008/3782/products/inception_advance_SD18120_B_2_framed1_57a8f726-e4da-4a60-877b-95b210b8fc91-367857.jpg?v=1611688027".into(),
        },
        Movie {
            id: "2".into(),
            title: "The Shawshank Redemption".into(),
            genres: vec!["Drama".into()],
            description: "Over the course of several years, two convicts form a friendship, seeking consolation and, eventually, redemption through basic compassion.".into(),
            poster_url: "https://i.etsystatic.com/16821137/r/il/c8b3e3/1879586236/il_570xN.1879586236_kdtm.jpg".into(),
        },
    ];

    let movie = Movie {
        id: "2".into(),
        title: "The Shawshank Redemption".into(),
        genres: vec!["Drama".into()],
        description: "Over the course of several years, two convicts form a friendship, seeking consolation and, eventually, redemption through basic compassion.".into(),
        poster_url: "https://i.etsystatic.com/16821137/r/il/c8b3e3/1879586236/il_570xN.1879586236_kdtm.jpg".into(),
    };

    Json(movie)
}

pub fn routes() -> Vec<Route> {
    routes![start, movies]
}