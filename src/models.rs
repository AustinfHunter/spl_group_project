use diesel::prelude::*;
use serde::{Serialize,Deserialize};
use rocket::FromForm;

// Track is used to instantiate Track objects
#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::Track)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Track {
    pub ID: i32,
    pub track_name:String,
    pub artist_name:String,
    pub artist_count:i8,
    pub release_year:i16,
    pub release_month:i8,
    pub release_day:i8,
    pub streams:i64,
    pub bpm:i16,
    pub danceability:i8,
    pub valence:i8,
    pub energy:i8,
    pub acousticness:i8,
    pub instrumentalness:i8,
    pub liveness:i8,
    pub speechiness:i8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackInsert {
    pub track_name: String,
    pub artist_name: String,
    pub artist_count: i8,
    pub release_year: i16,
    pub release_month: i8,
    pub release_day: i8,
    pub streams: i64,
    pub bpm: i16,
    pub danceability: i8,
    pub valence: i8,
    pub energy: i8,
    pub acousticness: i8,
    pub instrumentalness: i8,
    pub liveness: i8,
    pub speechiness: i8,
}


//SurveyResponse is used to store user responses to the musical taste survey
#[derive(FromForm)]
pub struct SurveyResponse {
    pub danceability: i8,
    pub valence: i8,
    pub energy: i8,
    pub acousticness: i8,
    pub instrumentalness: i8,
    pub liveness: i8,
    pub speechiness: i8,
}

impl SurveyResponse {
    pub fn new(danceability: i8, valence: i8, energy: i8, acousticness: i8, instrumentalness: i8, liveness: i8, speechiness: i8) -> SurveyResponse {
        SurveyResponse{danceability,valence,energy,acousticness,instrumentalness,liveness,speechiness}
    }
}
