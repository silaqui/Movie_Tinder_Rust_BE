use crate::model::movie::{Movie, MovieDetails};

pub fn get_movies() -> Vec<Movie> {
    vec![
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
        Movie {
            id: "3".into(),
            title: "The Dark Knight".into(),
            genres: vec!["Action".into(), "Crime".into(), "Drama".into()],
            description: "When the menace known as the Joker wreaks havoc and chaos on the people of Gotham, Batman must accept one of the greatest psychological and physical tests of his ability to fight injustice.".into(),
            poster_url: "https://media.wired.com/photos/5932936052d99d6b984df845/master/w_2560%2Cc_limit/dark-knight-poster.jpg".into(),
        },
        Movie {
            id: "4".into(),
            title: "Schindler's List".into(),
            genres: vec!["Biography".into(), "Drama".into(), "History".into()],
            description: "In German-occupied Poland during World War II, industrialist Oskar Schindler gradually becomes concerned for his Jewish workforce after witnessing their persecution by the Nazis.".into(),
            poster_url: "https://images.fandango.com/ImageRenderer/0/0/redesign/static/img/default_poster.png/0/images/masterrepository/Fandango/215803/SCHINDLERS-LIST-TEASER-ONE-SHEET-27x40.jpg".into(),
        },
        Movie {
            id: "5".into(),
            title: "12 Angry Men".into(),
            genres: vec!["Crime".into(), "Drama".into()],
            description: "The jury in a New York City murder trial is frustrated by a single member whose skeptical caution forces them to more carefully consider the evidence before jumping to a hasty verdict.".into(),
            poster_url: "https://static.displate.com/857x1200/displate/2020-10-29/3cb96a6c646cfc21056e44d1fdda502f_e111735e11d842e683b77fd5a9f6f41d.jpg".into(),
        },
        Movie {
            id: "6".into(),
            title: "The Lord of the Rings: The Return of the King".into(),
            genres: vec!["Action".into(), "Adventure".into(), "Drama".into()],
            description: "Gandalf and Aragorn lead the World of Men against Sauron's army to draw his gaze from Frodo and Sam as they approach Mount Doom with the One Ring.".into(),
            poster_url: "https://cdn.shopify.com/s/files/1/2250/7105/products/doaly-Lord-of-the-Ring-Return-of-the-King-movie-poster-foil-2021.jpg?v=1639781564".into(),
        },
        Movie {
            id: "7".into(),
            title: "The Godfather Part II".into(),
            genres: vec!["Crime".into(), "Drama".into()],
            description: "The early life and career of Vito Corleone in 1920s New York City is portrayed, while his son, Michael, expands and tightens his grip on the family crime syndicate.".into(),
            poster_url: "https://m.media-amazon.com/images/M/MV5BMWMwMGQzZTItY2JlNC00OWZiLWIyMDctNDk2ZDQ2YjRjMWQ0XkEyXkFqcGdeQXVyNzkwMjQ5NzM@._V1_.jpg".into(),
        },
        Movie {
            id: "8".into(),
            title: "Pulp Fiction".into(),
            genres: vec!["Crime".into(), "Drama".into()],
            description: "The lives of two mob hitmen, a boxer, a gangster and his wife, and a pair of diner bandits intertwine in four tales of violence and redemption.".into(),
            poster_url: "https://m.media-amazon.com/images/M/MV5BNGNhMDIzZTUtNTBlZi00MTRlLWFjM2ItYzViMjE3YzI5MjljXkEyXkFqcGdeQXVyNzkwMjQ5NzM@._V1_.jpg".into(),
        },
        Movie {
            id: "9".into(),
            title: "Fight Club".into(),
            genres: vec!["Drama".into()],
            description: "An insomniac office worker and a devil-may-care soap maker form an underground fight club that evolves into much more.".into(),
            poster_url: "https://m.media-amazon.com/images/M/MV5BNDIzNDU0YzEtYzE5Ni00ZjlkLTk5ZjgtNjM3NWE4YzA3Nzk3XkEyXkFqcGdeQXVyMjUzOTY1NTc@._V1_.jpg".into(),
        }
    ]
}


pub fn _get_movie_details() -> MovieDetails {
    MovieDetails {
        title: "The Shawshank Redemption".into(),
        position: "1".into(),
        year: 1994,
        certificate: "R".into(),
        runtime: "142".into(),
        genre: vec!["Drama".into()],
        description: "Over the course of several years, two convicts form a friendship, seeking consolation and, eventually, redemption through basic compassion.".into(),
        director: vec!["Frank Darabont".into()],
        stars: vec!["Tim Robbins".into(), "Morgan Freeman".into(), "Bob Gunton".into(), "William Sadler".into()],
        poster_url: "".into(),
    }
}
