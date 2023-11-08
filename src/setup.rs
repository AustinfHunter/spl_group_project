use crate::models::Track;
use std::error::Error;
use mysql::prelude::*;
use mysql::*;

/// setup_msql is a helper function that will read a csv file containing Spotify data formatted
/// like the Track struct in models.
pub fn setup_msql(mut conn: mysql::PooledConn) -> Result<(), Box<dyn Error>> {
    let mut rdr =  csv::Reader::from_reader(std::io::stdin());
    let mut tracks = Vec::<Track>::new();
    let mut tran = conn.start_transaction(TxOpts::default()).expect("error tran");
    for result in rdr.deserialize() {
       let track: Track = result?;
       tracks.push(track);
    }
    let res = tran.exec_batch(
    r"INSERT INTO Track (track_name,artist_name,artist_count,release_year,release_month,release_day,streams,bpm,danceability,valence,energy,acousticness,instrumentalness,liveness,speechiness)
    VALUES(:track_name,:artist_name,:artist_count,:release_year,:release_month,:release_day,:streams,:bpm,:danceability,:valence,:energy,:acousticness,:instrumentalness,:liveness,:speechiness)",    
    tracks.iter().map(|t| params!{
                    "track_name" => t.track_name.clone(),
                    "artist_name" => t.artist_name.clone(),
                    "artist_count" => t.artist_count,
                    "release_year" => t.release_year,
                    "release_month" => t.release_month,
                    "release_day" => t.release_day,
                    "streams" => t.streams,
                    "bpm" => t.bpm,
                    "danceability" => t.danceability,
                    "valence" => t.valence,
                    "energy" => t.energy,
                    "acousticness" => t.acousticness,
                    "instrumentalness" => t.instrumentalness,
                    "liveness" => t.liveness,
                    "speechiness" => t.speechiness}));
    res.expect("batch err");
    tran.commit().expect("commit err");
    Ok(())
}
