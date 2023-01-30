use crate::{
    profanity::check_profanity,
    store::Store,
    types::{
        pagination::{extract_pagination, Pagination},
        question::{NewQuestion, Question},
    },
};

use tracing::{event, instrument, Level};

use std::collections::HashMap;
use warp::http::StatusCode;

#[instrument]
pub async fn add_question(
    store: Store,
    new_question: NewQuestion,
) -> Result<impl warp::Reply, warp::Rejection> {
    let title = match check_profanity(new_question.title).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    let content = match check_profanity(new_question.content).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    let question = NewQuestion {
        title,
        content,
        tags: new_question.tags,
    };

    match store.add_question(question).await {
        Ok(question) => {
            event!(target: "warp-rest-api", Level::INFO, "POST NEW Question");
            Ok(warp::reply::json(&question))
        }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

#[instrument]
pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
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
        Ok(res) => {
            event!(target: "warp-rest-api", Level::INFO, "GET ALL Questions");
            Ok(warp::reply::json(&res))
        }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

#[instrument]
pub async fn get_question_by_id(
    id: i32,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.get_question_by_id(id).await {
        Ok(res) => {
            event!(target: "warp-rest-api", Level::INFO, "GET Question by ID");
            Ok(warp::reply::json(&res))
        }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

#[instrument]
pub async fn update_question(
    id: i32,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    let title = check_profanity(question.title);
    let content = check_profanity(question.content);

    let (title, content) = tokio::join!(title, content);

    if title.is_err() {
        return Err(warp::reject::custom(title.unwrap_err()));
    }

    if content.is_err() {
        return Err(warp::reject::custom(content.unwrap_err()));
    }

    let question = Question {
        id: question.id,
        title: title.unwrap(),
        content: content.unwrap(),
        tags: question.tags,
    };

    match store.update_question(question, id).await {
        Ok(res) => {
            event!(target: "warp-rest-api", Level::INFO, "UPDATE Question");
            Ok(warp::reply::json(&res))
        }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

#[instrument]
pub async fn delete_question(id: i32, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    match store.delete_question(id).await {
        Ok(_) => {
            event!(target: "warp-rest-api", Level::INFO, "DELETE Question");
            Ok(warp::reply::with_status(
                format!("Question {id} DELETED"),
                StatusCode::OK,
            ))
        }
        Err(e) => Err(warp::reject::custom(e)),
    }
}
