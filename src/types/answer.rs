use crate::types::question::QuestionId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct AnswerId(pub i32);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Answer {
    pub id: AnswerId,
    pub content: String,
    pub question_id: QuestionId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAnswer {
    pub content: String,
    pub question_id: QuestionId,
}
