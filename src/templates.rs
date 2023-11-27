use askama::Template;
use rocket::Responder;
use crate::models::Track;

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
#[template(path = "topTen.html")]
pub struct TopTenTemplate<'a>{
    pub tracks: &'a Vec<Track>,
}

#[derive(Template)]
#[template(path = "random.html")]
pub struct RandomTemplate<'a>{
    pub tracks: &'a Vec<Track>,
}
