#[derive(Queryable)]
pub struct Movie {
    pub id: i32,
    pub title: String,
    pub director: String,
    pub release_year: i32,
}

use super::schema::movies;

#[derive(Insertable)]
#[table_name="movies"]
pub struct NewMovie<'a> {
    pub title: &'a str,
    pub director: &'a str,
    pub release_year: &'a i32,
}