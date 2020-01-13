extern crate diesel_demo;
extern crate diesel;

use self::diesel_demo::*;
use std::io::stdin;



fn main() {
    let connection = establish_connection();

    println!("Title:");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)]; // Drop the newline character
    println!("Director:");
    let mut director = String::new();
    stdin().read_line(&mut director).unwrap();
    let director = &director[..(director.len() - 1)];
    println!("Release year:");
    let mut year = String::new();
    stdin().read_line(&mut year).unwrap();
    let year: i32 = year.trim().parse().unwrap();    
    let movie = create_movie(&connection, title, &director, &year);
    println!("\nSaved movie {} with id {}", title, movie.id);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";