use db::Conn as DbConn
use rocket_contrib::Json;
use super::models::{Book, NewBook};
use serde_json::Value;

#[get("/books", format = "application/json")]
fn index(conn: DbConn) -> Json<Value> {
    let books = Book::all(&conn);

    Json(json!({
        "status": 200,
        "result": books,
    }))
}

#[post("/books", format = "application/json", data = "<new_book>")]
fn new(conn: DbConn, new_book: Json<NewBook>) -> Json<Value> {
    Json(json!({
        "status": Book::insert(new_book.into_inner(), &conn),
        "result": Book::all(&conn).first(),
    }))
}
