use actix_web::web::Data;
use juniper::{EmptySubscription, FieldError, FieldResult, RootNode};

use crate::{
    create_todo, delete_todo_by_id, get_all_todos, get_todo_by_id,
    models::{CreateTodoInput, Todo, UpdateTodoInput},
    update_todo_by_id, AppState,
};

pub struct Query;

#[graphql_object(context = Data<AppState>)]
impl Query {
    fn api_version() -> &'static str {
        "1.0"
    }

    fn todos(context: &Data<AppState>) -> FieldResult<Vec<Todo>> {
        let mut connection = (*context).db_conn.lock().unwrap();

        match get_all_todos(&mut connection) {
            Ok(todos) => Ok(todos),
            Err(err) => Err(err.into()), // Convert the error to a FieldError for GraphQL
        }
    }

    fn todo(context: &Data<AppState>, id: i32) -> FieldResult<Todo> {
        let mut connection = (*context).db_conn.lock().unwrap();

        match get_todo_by_id(&mut connection, &id) {
            Some(todo) => Ok(todo),
            None => Err(FieldError::new(
                "Todo not found",
                graphql_value!({ "type": "NOT_FOUND" }),
            )),
        }
    }
}

pub struct Mutation;

#[graphql_object(context = Data<AppState>)]
impl Mutation {
    fn create_todo(context: &Data<AppState>, input: CreateTodoInput) -> FieldResult<Todo> {
        let mut connection = (*context).db_conn.lock().unwrap();

        match create_todo(&mut connection, &input.title) {
            Ok(todo) => Ok(todo),
            Err(err) => Err(err.into()),
        }
    }

    fn update_todo(context: &Data<AppState>, id: i32, input: UpdateTodoInput) -> FieldResult<Todo> {
        let mut connection = (*context).db_conn.lock().unwrap();

        match update_todo_by_id(&mut connection, &id, &input.completed) {
            Some(todo) => Ok(todo),
            None => Err(FieldError::new(
                "Todo not found",
                graphql_value!({ "type": "NOT_FOUND" }),
            )),
        }
    }

    fn delete_todo(context: &Data<AppState>, id: i32) -> FieldResult<bool> {
        let mut connection = (*context).db_conn.lock().unwrap();

        match delete_todo_by_id(&mut connection, &id) {
            Ok(deleted) => Ok(deleted),
            Err(err) => Err(err.into()),
        }
    }
}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Data<AppState>>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::<Data<AppState>>::new())
}
