use tracing::{event, instrument, Level};
use warp::http::StatusCode;

use crate::{
    profanity::check_profanity,
    store::Store,
    types::{account::Session, answer::NewAnswer},
};

#[instrument]
pub async fn add_answer(
    session: Session,
    store: Store,
    new_answer: NewAnswer,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;
    let content = match check_profanity(new_answer.content).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    let answer = NewAnswer {
        content,
        question_id: new_answer.question_id,
    };

    match store.add_answer(answer, account_id).await {
        Ok(_) => {
            event!(target: "warp-rest-api", Level::INFO, "POST NEW Answer");
            Ok(warp::reply::with_status("Answer added", StatusCode::OK))
        }
        Err(e) => Err(warp::reject::custom(e)),
    }
}
