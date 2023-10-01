use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Question {
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct QuestionDetail {
    pub question_uuid: String,
    pub title: String,
    pub description: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct QuestionId {
    pub question_uuid: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Answer {
    pub question_uuid: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AnswerDetail {
    pub answer_uuid: String,
    pub question_uuid: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AnswerId {
    pub answer_uuid: String,
}
