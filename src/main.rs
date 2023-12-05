use actix_cors::Cors;
use actix_web::{
    http::header,
    web::{self, Data},
    App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use std::{
    io,
    sync::{Arc, Mutex},
};

use juniper_actix::{graphiql_handler, graphql_handler, playground_handler};
use rust_graphql_todoapi::{
    establish_connection,
    graphql::schema::{create_schema, Schema},
    AppState,
};

/// GraphQL playground UI
async fn playground() -> Result<HttpResponse, Error> {
    playground_handler("/graphql", None).await
}

/// GraphiQL playground UI
async fn graphiql() -> impl Responder {
    graphiql_handler("/graphql", None).await
}

/// GraphQL endpoint
async fn graphql(
    req: HttpRequest,
    payload: web::Payload,
    schema: Data<Schema>,
) -> Result<HttpResponse, Error> {
    let connection = establish_connection();

    let context = web::Data::new(AppState {
        db_conn: Mutex::new(connection),
    });
    graphql_handler(&schema, &context, req, payload).await
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Create Juniper schema
    let schema = Arc::new(create_schema());

    // log::info!("starting HTTP server on port 8080");
    // log::info!("GraphiQL playground: http://localhost:8080/graphiql");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(Data::from(schema.clone()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["POST", "GET"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .service(
                web::resource("/graphql")
                    .route(web::post().to(graphql))
                    .route(web::get().to(graphql)),
            )
            .service(web::resource("/playground").route(web::get().to(playground)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
