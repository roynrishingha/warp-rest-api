use crate::{store::Store, types::answer::NewAnswer};
use tracing::{event, instrument, Level};
use warp::http::StatusCode;

#[instrument]
pub async fn add_answer(
    store: Store,
    new_answer: NewAnswer,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(target: "warp-rest-api", Level::INFO, "Adding new answer");

    match store.add_answer(new_answer).await {
        Ok(_) => Ok(warp::reply::with_status(
            "Answer added",
            StatusCode::CREATED,
        )),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
