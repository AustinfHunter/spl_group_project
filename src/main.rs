mod models;
mod schema;
mod data;

fn main() {
    let sur = models::SurveyResponse::new(2,0,0,0,0,0);
    data::analytics(&sur,None);
}
