use askama::Template;
use rocket::Responder;
use crate::{models::{Track, SurveyResponse}, data::Analytics};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub placeholder: &'a str,
}

pub struct Question<'a>{
    pub name: &'a str,
    pub value: &'a str,
    pub attr_type: &'a str,
}

#[derive(Template)]
#[template(path = "questionnaire.html")]
pub struct QuestionaireTemplate<'a> {
    pub questions: &'a Vec<Question<'a>>
}

#[derive(Template)]
#[template(path = "results.html")]
pub struct ResultListTemplate<'a>{
    pub tracks: &'a Vec<Track>,
}

#[derive(Template)]
#[template(path = "landing.html")]
pub struct LandingPageTemplate<'a>{
    pub analytics: &'a Analytics,
}
