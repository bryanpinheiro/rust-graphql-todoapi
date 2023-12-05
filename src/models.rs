use super::schema::todos;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, GraphQLObject)]
#[diesel(table_name = crate::schema::todos)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = todos)]
pub struct NewTodo<'a> {
    pub title: &'a str,
}

#[derive(Deserialize, GraphQLInputObject)]
pub struct CreateTodoInput {
    pub title: String,
}

#[derive(Deserialize, GraphQLInputObject)]
pub struct UpdateTodoInput {
    pub completed: bool,
}
