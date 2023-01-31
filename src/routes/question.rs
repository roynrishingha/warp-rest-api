use tracing::{event, instrument, Level};

use std::collections::HashMap;
use warp::http::StatusCode;

use crate::{
    profanity::check_profanity,
    store::Store,
    types::{
        account::Session,
        pagination::{extract_pagination, Pagination},
        question::{NewQuestion, Question},
    },
};

#[instrument]
pub async fn add_question(
    session: Session,
    store: Store,
    new_question: NewQuestion,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;
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

    match store.add_question(question, account_id).await {
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
    session: Session,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;
    if store.is_question_owner(id, &account_id).await? {
        let title = check_profanity(question.title);
        let content = check_profanity(question.content);

        let (title, content) = tokio::join!(title, content);

        if title.is_ok() && content.is_ok() {
            let question = Question {
                id: question.id,
                title: title.unwrap(),
                content: content.unwrap(),
                tags: question.tags,
            };
            match store.update_question(question, id, account_id).await {
                Ok(res) => {
                    event!(target: "warp-rest-api", Level::INFO, "UPDATE Question");
                    Ok(warp::reply::json(&res))
                }
                Err(e) => Err(warp::reject::custom(e)),
            }
        } else {
            Err(warp::reject::custom(
                title.expect_err("Expected API call to have failed here"),
            ))
        }
    } else {
        Err(warp::reject::custom(handle_errors::Error::Unauthorized))
    }
}

#[instrument]
pub async fn delete_question(
    id: i32,
    session: Session,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;
    if store.is_question_owner(id, &account_id).await? {
        match store.delete_question(id, account_id).await {
            Ok(_) => {
                event!(target: "warp-rest-api", Level::INFO, "DELETE Question");
                Ok(warp::reply::with_status(
                    format!("Question {} deleted", id),
                    StatusCode::OK,
                ))
            }
            Err(e) => Err(warp::reject::custom(e)),
        }
    } else {
        Err(warp::reject::custom(handle_errors::Error::Unauthorized))
    }
}
