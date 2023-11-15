// @generated automatically by Diesel CLI.

diesel::table! {
    Track (ID) {
        ID -> Integer,
        #[max_length = 250]
        track_name -> Varchar,
        #[max_length = 250]
        artist_name -> Varchar,
        artist_count -> Tinyint,
        release_year -> Smallint,
        release_month -> Tinyint,
        release_day -> Tinyint,
        streams -> Bigint,
        bpm -> Smallint,
        danceability -> Tinyint,
        valence -> Tinyint,
        energy -> Tinyint,
        acousticness -> Tinyint,
        instrumentalness -> Tinyint,
        liveness -> Tinyint,
        speechiness -> Tinyint,
    }
}
