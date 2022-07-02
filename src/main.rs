#[macro_use]
extern crate diesel;

use diesel::{insert_into, prelude::*};
use dotenv::dotenv;
use std::env;

use self::models::*;
pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not defined!");

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    use self::schema::todos::dsl::*;

    let connection = establish_connection();

    let insert_res = insert_into(todos)
        .values((
            id.eq(1),
            title.eq("My first todo"),
            description.eq("This is my first todo"),
            is_done.eq(false),
        ))
        .execute(&connection);

    match insert_res {
        Ok(_) => println!("Inserted a new todo!"),
        Err(e) => println!("Error inserting a new todo: {:?}", e),
    }

    // SELECT
    let results = todos
        .limit(5)
        .load::<Todo>(&connection)
        .expect("Error loading todos");

    for todo in results {
        println!("id: {}", todo.id);
        println!("title: {}", todo.title);
        println!("description: {}", todo.description);
        println!("is_done: {}", todo.is_done);
    }
}
