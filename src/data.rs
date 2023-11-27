use crate::models;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use rand::Rng;
use rand::distributions::{Distribution,Uniform};

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

pub fn get_top_ten(limit: Option<i64>) -> Vec<models::Track> {
    use crate::schema::Track::dsl::*;
    let lim = limit.unwrap_or(25);
    let conn = &mut get_connection();

    Track
        .order(streams.desc())
        .limit(lim)
        .select(models::Track::as_select())
        .load(conn)
        .expect("Error loading tracks")
}

pub fn get_random(limit: Option<i64>) -> Vec<models::Track> {
    use crate::schema::Track::dsl::*;
    let lim = limit.unwrap_or(25);
    let conn = &mut get_connection();
    let num = rand::thread_rng().gen_range(0..954);

    Track
        .filter(ID.gt(num))
        .limit(lim)
        .select(models::Track::as_select())
        .load(conn)
        .expect("Error loading tracks")
}

// Gets Tracks using get_curated_tracks, then displays the tracks along with some basic statistics
// from the collection of Tracks
pub fn analytics(survey_response: &models::SurveyResponse, limit: Option<i64>) {
    let res = get_curated_tracks(survey_response,limit);
    let mut len = res.len() as i32;

    let (mut dance,mut val,mut ener,mut acoust,mut instrum,mut live,mut speech,mut bpm) : (i32,i32,i32,i32,i32,i32,i32,i32) = (0,0,0,0,0,0,0,0);
    println!("Displaying {} tracks\n---------------", res.len());
    for track in res {
        println!("Artist: {}", track.artist_name);
        println!("Track Name: {}", track.track_name);
        println!("Streams: {}\n", track.streams);
        dance += track.danceability as i32;
        val += track.valence as i32;
        ener += track.energy as i32;
        acoust += track.acousticness as i32;
        live += track.liveness as i32;
        instrum += track.instrumentalness as i32;
        speech += track.speechiness as i32;
        bpm += track.bpm as i32;

    }
    println!("---------------");
    println!("Total Songs: {}", len);
    if len == 0 {
        len = 1
    }
    println!("Mean Danceability: {}%", dance/len);
    println!("Mean Valence: {}%", val/len);
    println!("Mean Energy: {}%", ener/len);
    println!("Mean Acousticness: {}%", acoust/len);
    println!("Mean Liveness: {}%", live/len);
    println!("Mean Instrumentalness: {}%", instrum/len);
    println!("Mean Speechiness: {}%", speech/len);
    println!("Mean Beats Per Minute: {}bpm", bpm/len);
}

pub struct Analytics {
    pub total_songs: i32,
    pub danceability: i32,
    pub valence: i32,
    pub energy: i32,
    pub acousticness: i32,
    pub liveness: i32,
    pub instrumentalness: i32,
    pub speechiness: i32,
    pub bpm: i32,
}

pub fn get_analytics(survey_response: &models::SurveyResponse, limit: Option<i64>) -> Analytics {
    let res = get_curated_tracks(survey_response,limit);
    let mut len = res.len() as i32;

    let (mut dance,mut val,mut ener,mut acoust,mut instrum,mut live,mut speech,mut bpm) : (i32,i32,i32,i32,i32,i32,i32,i32) = (0,0,0,0,0,0,0,0);
    println!("Displaying {} tracks\n---------------", res.len());
    for track in res {
        println!("Artist: {}", track.artist_name);
        println!("Track Name: {}", track.track_name);
        println!("Streams: {}\n", track.streams);
        dance += track.danceability as i32;
        val += track.valence as i32;
        ener += track.energy as i32;
        acoust += track.acousticness as i32;
        live += track.liveness as i32;
        instrum += track.instrumentalness as i32;
        speech += track.speechiness as i32;
        bpm += track.bpm as i32;

    }
    println!("---------------");
    println!("Total Songs: {}", len);
    if len == 0 {
        len = 1
    }

    Analytics{
        total_songs: len,
        danceability:  dance/len,
        valence: val/len,
        energy: ener/len,
        acousticness: acoust/len,
        liveness: live/len,
        instrumentalness: instrum/len,
        speechiness: speech/len,
        bpm: bpm/len,
    }
}

pub fn result_distribution(num_samples: usize, limit: Option<i64>) {
    let mut i = 0;
    let mut rng = rand::thread_rng();
    let ans = Uniform::from(-2..2);
    let mut len : usize;
    let (mut zero, mut lt_ten, mut lt_fiftn, mut lt_twenty, mut lt_25, mut gt_25) : (f64,f64,f64,f64,f64,f64) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let (mut zero_speechy,mut zero_instrum) : (f64,f64) = (0.0,0.0);
    let mut total_songs : i64 = 0;
    let num_tests = num_samples as f64;
    while i < num_samples {
        let sur = models::SurveyResponse::new(
            ans.sample(&mut rng),
            ans.sample(&mut rng),
            ans.sample(&mut rng),
            ans.sample(&mut rng),
            ans.sample(&mut rng),
            ans.sample(&mut rng),
            ans.sample(&mut rng)
        );
        let res = get_curated_tracks(&sur,limit);
        len = res.len();
        if len == 0 {
            zero += 1.0;
            zero_speechy += sur.speechiness as f64;
            zero_instrum += sur.instrumentalness as f64;
        } else if len < 10 {
            lt_ten += 1.0;
        } else if len < 15 {
            lt_fiftn += 1.0;
        } else if len < 20 {
            lt_twenty += 1.0;
        } else if len < 25 {
            lt_25 += 1.0;
        } else {
            gt_25 += 1.0;
        }
        total_songs += len as i64;
        i += 1;
    }

    println!("----------------");
    println!("Distribution of result set size (number of matching songs) for {} randomly generated survey results:", num_samples);
    println!("No songs in result set: {}%", 100.0*zero/num_tests); 
    println!("0 to 9 songs in result set: {}%", 100.0*lt_ten/num_tests); 
    println!("10 to 14 songs in result set: {}%", 100.0*lt_fiftn/num_tests); 
    println!("15 to 19 songs in result set: {}%", 100.0*lt_twenty/num_tests); 
    println!("20 to 24 songs in result set: {}%", 100.0*lt_25/num_tests); 
    println!("Greater than 25 songs in result set: {}%", 100.0*gt_25/num_tests);
    println!("Mean size of result sets: {:.2} songs", (total_songs as f64)/num_tests);
    println!("--------------");
    println!("Why the empty result sets?");
    println!("Mean speechiness of sets with no results: {:.2}%", 100.0*zero_speechy/zero);
    println!("Mean instrumentalness of sets with no results: {:.2}%", 100.0*zero_instrum/zero);
}

