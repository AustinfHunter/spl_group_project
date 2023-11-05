use crate::models;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

// Helper function for calculating left boundary of SQL queries
fn left_bound(x: i8) -> i8 {
    if x < 0 {
        0
    } else {
        x
    }
}

// Helper function for calculatin right boundary of SQL queries
fn right_bound(x: i8) -> i8 {
   if x > 0 {
       0
   } else {
       x
   }
}


// Returns a connection to the MySQL database
fn get_connection() -> MysqlConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&db_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}

// Returns a Vec of Track objects
pub fn get_tracks(limit: Option<i64>) -> Vec<models::Track> {
   use crate::schema::Track::dsl::*;
   let lim = limit.unwrap_or(25);
   let conn = &mut get_connection();

   Track
       .limit(lim)
       .select(models::Track::as_select())
       .load(conn)
       .expect("Error loading tracks")
}

// Returns a Vec of Track objects filtered such that all columns are within the ranges calculated
// from the fields of the survey_response parameter
pub fn get_curated_tracks(survey_response: &models::SurveyResponse, limit: Option<i64>) -> Vec<models::Track> {
    use crate::schema::Track::dsl::*;
    let conn = &mut get_connection();
    let lim = limit.unwrap_or(25);

    Track
        .filter(instrumentalness.between(left_bound(survey_response.instrumentalness*25),100 + right_bound(survey_response.instrumentalness*25)))
        .filter(speechiness.between(left_bound(survey_response.speechiness*25),100 + right_bound(survey_response.speechiness*25)))
        .filter(liveness.between(left_bound(survey_response.liveness*25),100 + right_bound(survey_response.liveness*25)))
        .filter(acousticness.between(left_bound(survey_response.acousticness*25),100 + right_bound(survey_response.acousticness*25)))
        .filter(valence.between(left_bound(survey_response.valence*25),100 + right_bound(survey_response.valence*25)))
        .filter(danceability.between(left_bound(survey_response.danceability*25),100 + right_bound(survey_response.danceability*25)))
        .limit(lim)
        .select(models::Track::as_select())
        .load(conn)
        .expect("Error loading curated tracks")
}

// Gets Tracks using get_curated_tracks, then displays the tracks along with some basic statistics
// from the collection of Tracks
pub fn analytics(survey_response: &models::SurveyResponse, limit: Option<i64>) {
    let res = get_curated_tracks(survey_response,limit);
    let len = res.len() as i32;
    let (mut dance, mut val, mut acoust, mut instrum, mut live, mut speech,mut bpm) : (i32,i32,i32,i32,i32,i32,i32) = (0,0,0,0,0,0,0);
    println!("Displaying {} tracks\n---------------", res.len());
    for track in res {
        println!("Artist: {}", track.artist_name.unwrap());
        println!("Track Name: {}", track.track_name.unwrap());
        println!("Streams: {}\n", track.streams.unwrap());
        dance += track.danceability.unwrap_or(0) as i32;
        val += track.valence.unwrap_or(0) as i32;
        acoust += track.acousticness.unwrap_or(0) as i32;
        live += track.liveness.unwrap_or(0) as i32;
        instrum += track.instrumentalness.unwrap_or(0) as i32;
        speech += track.speechiness.unwrap_or(0) as i32;
        bpm += track.bpm.unwrap_or(0) as i32;

    }
    println!("---------------");
    println!("Total Songs: {}", len);
    println!("Mean Danceability: {}%", dance/len);
    println!("Mean Valence: {}%", val/len);
    println!("Mean Acousticness: {}%", acoust/len);
    println!("Mean Liveness: {}%", live/len);
    println!("Mean Instrumentalness: {}%", instrum/len);
    println!("Mean Speechiness: {}%", speech/len);
    println!("Mean Beats Per Minute: {}bpm", bpm/len);
}
