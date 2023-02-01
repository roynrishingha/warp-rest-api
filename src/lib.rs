#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(clippy::module_name_repetitions)]
#![allow(opaque_hidden_inferred_bound)]

pub use handle_errors;

use crate::routes::{answer, authentication, question};
use tokio::sync::{oneshot, oneshot::Sender};
use tracing_subscriber::fmt::format::FmtSpan;
use warp::{http::Method, Filter, Reply};

pub mod config;
mod profanity;
mod routes;
mod store;
pub mod types;

pub struct OneshotHandler {
    pub sender: Sender<i32>,
}

async fn build_routes(store: store::Store) -> impl Filter<Extract = impl Reply> + Clone {
    let store_filter = warp::any().map(move || store.clone());

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

    add_question
        .or(get_questions)
        .or(get_one_question)
        .or(update_question)
        .or(delete_question)
        .or(add_answer)
        .or(registration)
        .or(login)
        .with(cors)
        .with(warp::trace::request())
        .recover(handle_errors::return_error)
}

pub async fn setup_store(config: &config::Config) -> Result<store::Store, handle_errors::Error> {
    let store = store::Store::new(&format!(
        "postgres://{}:{}@{}:{}/{}",
        config.db_user, config.db_password, config.db_host, config.db_port, config.db_name
    ))
    .await
    .map_err(handle_errors::Error::DatabaseQueryError)?;

    sqlx::migrate!()
        .run(&store.clone().connection)
        .await
        .map_err(handle_errors::Error::MigrationError)?;

    let log_filter = format!(
        "handle_errors={},warp-rest-api={},warp={}",
        config.log_level, config.log_level, config.log_level
    );

    tracing_subscriber::fmt()
        // Use the filter we built above to determine which traces to record.
        .with_env_filter(log_filter)
        // Record an event when each span closes. This can be used to time our
        // routes' durations!
        .with_span_events(FmtSpan::CLOSE)
        .init();

    Ok(store)
}

pub async fn run(config: config::Config, store: store::Store) {
    let routes = build_routes(store).await;
    warp::serve(routes).run(([0, 0, 0, 0], config.port)).await;
}

pub async fn oneshot(store: store::Store) -> OneshotHandler {
    let routes = build_routes(store).await;
    let (tx, rx) = oneshot::channel::<i32>();

    let socket: std::net::SocketAddr = "127.0.0.1:3030"
        .to_string()
        .parse()
        .expect("Not a valid address");

    let (_, server) = warp::serve(routes).bind_with_graceful_shutdown(socket, async {
        rx.await.ok();
    });

    tokio::task::spawn(server);

    OneshotHandler { sender: tx }
}
