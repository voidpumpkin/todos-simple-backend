use actix_cors::Cors;
use actix_web::{App, HttpServer};
use diesel::sqlite::SqliteConnection;
use diesel_migrations::embed_migrations;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use std::env;
use std::ops::Deref;

use crate::controllers::delete_todo;
use crate::controllers::delete_todos;
use crate::controllers::get_todos;
use crate::controllers::patch_todo;
use crate::controllers::post_todo;

#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate diesel;

pub mod controllers;
pub mod models;
pub mod schema;

embed_migrations!("./migrations");

const PORT: &str = "8080";

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let arg_port: String = args.get(1).cloned().unwrap_or_else(|| PORT.to_string());
    let port = if arg_port.is_empty() {
        PORT.to_string()
    } else {
        arg_port
    };

    let skip_test_data = args
        .get(2)
        .cloned()
        .unwrap_or_else(|| PORT.to_string())
        .eq("true");

    let manager = ConnectionManager::<SqliteConnection>::new("todos.db");
    let pool = Pool::new(manager)?;

    // Run migrations
    let db = pool.get()?;
    embedded_migrations::run_with_output(db.deref(), &mut std::io::stdout())?;

    if skip_test_data {
        diesel_migrations::revert_latest_migration(db.deref())?;
    }

    println!("listening on 0.0.0.0:{port}");

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .data(pool.clone())
            .wrap(cors)
            .service(get_todos)
            .service(post_todo)
            .service(patch_todo)
            .service(delete_todo)
            .service(delete_todos)
    })
    .bind(format!("0.0.0.0:{port}"))?
    .run()
    .await?;

    Ok(())
}
