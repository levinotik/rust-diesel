pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

use self::models::{Movie, NewMovie};

pub fn create_movie<'a>(conn: &PgConnection, title: &'a str, director: &'a str, release_year: &'a i32) -> Movie {
    use schema::movies;

    let new_movie = NewMovie {
        title: title,
        director: director,
        release_year: release_year
    };

    diesel::insert_into(movies::table)
        .values(&new_movie)
        .get_result(conn)
        .expect("Error saving new movie")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}