// TODO: import log, pretty_env_logger, dotenv, and PgPoolOptions

mod cors;
mod handlers;
mod models;

#[macro_use]
extern crate rocket;

use cors::*;
use handlers::*;
use log::info;
use sqlx::postgres::PgPoolOptions;

#[launch]
async fn rocket() -> _ {
    pretty_env_logger::init();
    dotenvy::dotenv().expect("Failed to initialize dotenvy.");

    let database_url =
        dotenvy::var("DATABASE_URL").expect("Missing DATABASE_URL environment variable.");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database.");

    // Test
    let records = sqlx::query!("SELECT * FROM questions")
        .fetch_all(&pool)
        .await
        .expect("Failed to query database.");
    info!("********* Question Records *********");
    for record in records {
        info!("{record:?}");
    }

    rocket::build()
        .mount(
            "/",
            routes![
                create_question,
                read_questions,
                delete_question,
                create_answer,
                read_answers,
                delete_answer
            ],
        )
        .attach(Cors)
}
