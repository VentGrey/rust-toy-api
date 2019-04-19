use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use schema::books;
use schema::books::dsl::books as all_books;

#[derive(Queryable)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub published: bool,
}


#[derive(Insertable)]
#[table_name = "books"]
pub struct NewBook {
    pub title: String,
    pub author: String,
    pub published: bool,
}

impl Book {
    pub fn show(id: i32, conn: &PgConnection) -> {
        all_books
            .find(id)
            .load::<Book>(conn)
            .expect("Error loading book")
    }

    pub fn all(conn: &PgConnection) -> Vec<Book> {
        all_books
            .order(books::id.desc())
            .load::<Book>(conn)
            .expect("Error loading book")
    }
}
