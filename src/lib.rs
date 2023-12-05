pub mod models;
pub mod schema;

#[macro_use]
extern crate juniper;
use diesel::prelude::*;
use diesel::result::Error;
use dotenv::dotenv;
use std::env;
use std::sync::Mutex;

use crate::models::{NewTodo, Todo};

pub struct AppState {
    pub db_conn: Mutex<SqliteConnection>,
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_todo(conn: &mut SqliteConnection, title: &str) -> Result<Todo, Error> {
    use crate::schema::todos;

    let new_todo = NewTodo { title };

    diesel::insert_into(todos::table)
        .values(&new_todo)
        .returning(Todo::as_returning())
        .get_result(conn)
}

pub fn get_all_todos(conn: &mut SqliteConnection) -> Result<Vec<Todo>, Error> {
    use self::schema::todos::dsl::*;

    todos.select(Todo::as_select()).load(conn)
}

pub fn get_todo_by_id(conn: &mut SqliteConnection, _id: &i32) -> Option<Todo> {
    use self::schema::todos::dsl::*;

    match todos.find(_id).first(conn) {
        Ok(todo) => Some(todo),
        Err(Error::NotFound) => None,
        Err(err) => {
            eprintln!("Error retrieving todo by id: {}", err);
            None
        }
    }
}

pub fn update_todo_by_id(
    conn: &mut SqliteConnection,
    _id: &i32,
    _completed: &bool,
) -> Option<Todo> {
    use self::schema::todos::dsl::*;

    match diesel::update(todos.find(_id))
        .set(completed.eq(_completed))
        .returning(Todo::as_returning())
        .get_result(conn)
    {
        Ok(todo) => Some(todo),
        Err(Error::NotFound) => None,
        Err(err) => {
            eprintln!("Error updating todo by id: {}", err);
            None
        }
    }
}

pub fn delete_todo_by_id(
    conn: &mut SqliteConnection,
    _id: &i32,
) -> Result<bool, diesel::result::Error> {
    use self::schema::todos::dsl::*;

    let rows_affected = diesel::delete(todos.find(_id)).execute(conn)?;

    Ok(rows_affected > 0)
}
