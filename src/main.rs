use crate::repository::prelude::TodosRepository;
use actix_web::{
    middleware,
    web::{self, Data},
    App, HttpServer,
};
use handler::prelude::todos_handler;

mod entity;
mod handler;
mod repository;

#[derive(Debug, Clone)]
pub struct AppState {
    todo_repository: TodosRepository,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ENV Setup
    std::env::set_var("RUST_LOG", "debug");
    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_conn = sea_orm::Database::connect(&db_url).await.unwrap();
    let host = std::env::var("HOST").expect("HOST is not set in .env file");
    let port = std::env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{}:{}", host, port);

    // App state setup
    let todo_repository = TodosRepository {
        db_conn: db_conn.clone(),
    };
    let state = AppState { todo_repository };

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .wrap(middleware::Logger::default())
            .configure(init)
    })
    .bind(&server_url)?;

    println!("Starting server at {}", server_url);
    server.run().await?;
    Ok(())
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(todos_handler());
}
