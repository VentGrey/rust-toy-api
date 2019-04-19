#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;

use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;


mod models;
mod schema;

fn main(){
    dotenv().ok(); // Because LAZY ERROR HANDLER

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let conn = PgConnection::establish(&database_url).unwrap();

    let book = models::NewBook {
        title: String::from("The Necronomicon"),
        author: String::from("Howard Phillips Lovecraft"),
        published: true,
    };

}
