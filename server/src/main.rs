use actix_web::{web, App, HttpServer};

use crate::database::Context;

#[actix_web::main]
async fn main() {
    let redis_url: &str = "";
    let cassandra_url: &str = "";

    // Initialize context
    let mut context = Context {
        redis_url,
        cassandra_url,
    };

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .data(context.clone())
            .service(get_message)
            .service(create_message)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
