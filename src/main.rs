#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(clippy::module_name_repetitions)]

mod config;
mod profanity;
mod routes;
mod store;
mod types;

use crate::{
    routes::{answer, authentication, question},
    store::Store,
};
use handle_errors::return_error;

use tracing_subscriber::fmt::format::FmtSpan;
use warp::{http::Method, Filter};

#[tokio::main]
async fn main() -> Result<(), handle_errors::Error> {
    let config = config::Config::new().expect("FAILED to set CONFIG");

    let log_filter = format!(
        "handle_errors={},warp-rest-api={},warp={}",
        config.log_level, config.log_level, config.log_level
    );

    // "postgres://username:password@localhost:5432/database_name"
    let store = Store::new(&format!(
        "postgresql://{}:{}@{}:{}/{}",
        config.db_user, config.db_password, config.db_host, config.db_port, config.db_name
    ))
    .await
    .map_err(|e| handle_errors::Error::DatabaseQueryError(e))?;

    sqlx::migrate!()
        .run(&store.clone().connection)
        .await
        .expect("Cannot run migration");

    let store_filter = warp::any().map(move || store.clone());

    tracing_subscriber::fmt()
        // Use the filter above to determine which traces to record.
        .with_env_filter(log_filter)
        // Record an event when each span closes.
        // This can be used to time
        // our routes' durations!
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(question::get_questions);

    let get_one_question = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(question::get_question_by_id);

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(authentication::auth())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(question::add_question);

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(authentication::auth())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(question::update_question);

    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(authentication::auth())
        .and(store_filter.clone())
        .and_then(question::delete_question);

    let add_answer = warp::post()
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(authentication::auth())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(answer::add_answer);

    let registration = warp::post()
        .and(warp::path("registration"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(authentication::register);

    let login = warp::post()
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(authentication::login);

    let routes = add_question
        .or(get_questions)
        .or(get_one_question)
        .or(update_question)
        .or(delete_question)
        .or(add_answer)
        .or(registration)
        .or(login)
        .with(cors)
        .with(warp::trace::request())
        .recover(return_error);

    tracing::info!("warp-rest-api build ID {}", env!("WARP_REST_API_VERSION"));

    warp::serve(routes).run(([0, 0, 0, 0], config.port)).await;

    Ok(())
}
