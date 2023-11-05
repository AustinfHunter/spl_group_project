use diesel::prelude::*;

// Track is used to instantiate Track objects
#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::Track)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Track {
    pub ID: i32,
    pub track_name: Option<String>,
    pub artist_name: Option<String>,
    pub artist_count: Option<i8>,
    pub release_year: Option<i16>,
    pub release_month: Option<i8>,
    pub release_day: Option<i8>,
    pub streams: Option<i64>,
    pub bpm: Option<i16>,
    pub danceability: Option<i8>,
    pub valence: Option<i8>,
    pub acousticness: Option<i8>,
    pub instrumentalness: Option<i8>,
    pub liveness: Option<i8>,
    pub speechiness: Option<i8>,
}

// SurveyResponse is used to store user responses to the musical taste survey
pub struct SurveyResponse {
    pub danceability: i8,
    pub valence: i8,
    pub acousticness: i8,
    pub instrumentalness: i8,
    pub liveness: i8,
    pub speechiness: i8,
}

impl SurveyResponse {
    pub fn new(danceability: i8, valence: i8, acousticness: i8, instrumentalness: i8, liveness: i8, speechiness: i8) -> SurveyResponse {
        SurveyResponse{danceability,valence,acousticness,instrumentalness,liveness,speechiness}
    }
}
