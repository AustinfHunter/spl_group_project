// @generated automatically by Diesel CLI.

diesel::table! {
    Track (ID) {
        ID -> Integer,
        #[max_length = 250]
        track_name -> Nullable<Varchar>,
        #[max_length = 250]
        artist_name -> Nullable<Varchar>,
        artist_count -> Nullable<Tinyint>,
        release_year -> Nullable<Smallint>,
        release_month -> Nullable<Tinyint>,
        release_day -> Nullable<Tinyint>,
        streams -> Nullable<Bigint>,
        bpm -> Nullable<Smallint>,
        danceability -> Nullable<Tinyint>,
        valence -> Nullable<Tinyint>,
        energy -> Nullable<Tinyint>,
        acousticness -> Nullable<Tinyint>,
        instrumentalness -> Nullable<Tinyint>,
        liveness -> Nullable<Tinyint>,
        speechiness -> Nullable<Tinyint>,
    }
}
