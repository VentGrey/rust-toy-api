#![feature(plugin, custom_derive, const_fn, decl_macro)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;


mod models;
mod schema;

fn main() {
    dotenv().ok(); // Because LAZY ERROR HANDLER

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let conn = PgConnection::establish(&database_url).unwrap();

    let book = models::NewBook {
        title: String::from("The Necronomicon"),
        author: String::from("Howard Phillips Lovecraft"),
        published: true,
    };

    if models::Book::insert(book, &conn) {
        println!("Success");
    } else {
        println!("FAILURE");
    }
}
