#[macro_use] extern crate rocket;
mod models;
mod schema;
mod data;
mod setup;
mod templates;
use std::env;
use data::get_curated_tracks;
use dotenvy::dotenv;
use models::SurveyResponse;
use mysql::*;
use rocket::form::Form;
use rocket::fs::{FileServer, relative};
use rocket::http::ContentType;
use askama::Template;
use templates::{IndexTemplate, QuestionaireTemplate, Question, ResultListTemplate};

fn handle_setup(args: Vec<String>) {
    if args.len() > 1 && args[1] == "setup" {
        println!("Starting db setup...");
        dotenv().ok();
        let db_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set in .env file. Example:  DATABASE_URL=mysql://uname:pass@localhost:3306/dbname");
        let opts = Opts::from_url(&db_url).unwrap();
        let pool = Pool::new(opts);
        let conn = pool.expect("could not establish mysql connection pool").get_conn().unwrap();
        let res = setup::setup_msql(conn).err();
        if res.is_some() {
            panic!("Error setting up database: {}", res.unwrap());
        }
    }
    data::analytics(&models::SurveyResponse::new(0,0,0,0,0,0,0),Some(1000));
    data::result_distribution(1000, Some(1000));
    return;
}


#[rocket::get("/")]
fn index() ->  (ContentType, String){
    let res = templates::IndexTemplate{placeholder: "placeholder from askama"};
    (ContentType::HTML,  res.render().unwrap())
}

#[rocket::get("/")]
fn questionnaire() ->  (ContentType, String){
    let questions = vec![
        Question{name: "danceability", value: "I like music I can dance to", attr_type: "danceability"},
        Question{name: "valence", value: "I like to listen to happy songs", attr_type: "valence"},
        Question{name: "energy", value: "I like music with a lot of energy", attr_type: "energy"},
        Question{name: "acousticness", value: "I prefer music with a lot of acoustic instruments", attr_type: "acousticness"},
        Question{name: "instrumentalness", value:"I prefer instrumental music", attr_type: "instrumentalness"},
        Question{name: "liveness", value: "I would rather listen to a live recording of a song than the album version", attr_type: "liveness"},
        Question{name: "speechiness", value: "I like music that focuses on lyrical content more than instrumentation", attr_type: "speechiness"},
    ];
    let res = templates::QuestionaireTemplate{questions: &questions};
    (ContentType::HTML,  res.render().unwrap())
}

#[rocket::post("/", data="<survey_form>")]
fn questionnaire_resp(survey_form: Form<SurveyResponse>) -> rocket::response::content::RawHtml<String> {
    //Hacky way to get form data, could have used JSON, didn't feel like messing with JS lol
    let surv = SurveyResponse::new(
        survey_form.danceability, 
        survey_form.valence, 
        survey_form.energy, 
        survey_form.acousticness, 
        survey_form.instrumentalness, 
        survey_form.liveness, 
        survey_form.speechiness);
    let tracks = get_curated_tracks(&surv, Some(15));
    let res = ResultListTemplate{tracks: &tracks};
    rocket::response::content::RawHtml(res.render().unwrap())
}


#[launch]
fn rocket() -> _ {
    //test askama
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "setup" {
        handle_setup(args);
    }
    rocket::build()
        .mount("/", rocket::routes![index])
        .mount("/questionnaire", rocket::routes![questionnaire,questionnaire_resp])
        .mount("/img", FileServer::from(relative!("static/Images")).rank(0))
}
