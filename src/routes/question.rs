use crate::{
    store::Store,
    types::{
        pagination::{extract_pagination, Pagination},
        question::{NewQuestion, Question},
    },
};

// use handle_errors::Error;
use tracing::{event, instrument, Level};

use std::collections::HashMap;
use warp::http::StatusCode;

#[instrument]
pub async fn add_question(
    store: Store,
    new_question: NewQuestion,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(target: "warp-rest-api", Level::INFO, "Adding new question");
    match store.add_question(new_question).await {
        Ok(_) => Ok(warp::reply::with_status(
            "Question Added",
            StatusCode::CREATED,
        )),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

#[instrument]
pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(target: "warp-rest-api", Level::INFO, "Querying questions");

    // Creates a mutable variable with the
    // default parameter for Pagination
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        event!(Level::INFO, pagination = true);

        // In case the pagination object is not empty,
        // we override our mutable variable from above
        // and replace it with the given Pagination
        // from the client.
        pagination = extract_pagination(&params)?;
    }

    match store
        .get_questions(pagination.limit, pagination.offset)
        .await
    {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

#[instrument]
pub async fn get_question_by_id(
    id: i32,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(target: "warp-rest-api", Level::INFO, "Querying question by id");
    match store.get_question_by_id(id).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

#[instrument]
pub async fn update_question(
    id: i32,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(target: "warp-rest-api", Level::INFO, "Updating question by id");
    match store.update_question(question, id).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

#[instrument]
pub async fn delete_question(id: i32, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    event!(target: "warp-rest-api", Level::INFO, "Deleting question by id");
    match store.delete_question(id).await {
        Ok(_) => Ok(warp::reply::with_status(
            format!("Question {id} deleted"),
            StatusCode::OK,
        )),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
