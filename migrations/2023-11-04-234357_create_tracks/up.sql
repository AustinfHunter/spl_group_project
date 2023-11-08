CREATE TABLE Track(
  ID int NOT NULL AUTO_INCREMENT,
  track_name varchar(250) NOT NULL, 
  artist_name varchar(250) NOT NULL,
  artist_count tinyint NOT NULL,
  release_year smallint NOT NULL,
  release_month tinyint NOT NULL,
  release_day tinyint NOT NULL,
  streams bigint NOT NULL,
  bpm smallint NOT NULL,
  danceability tinyint NOT NULL,
  valence tinyint NOT NULL,
  energy tinyint NOT NULL,
  acousticness tinyint NOT NULL,
  instrumentalness tinyint NOT NULL,
  liveness tinyint NOT NULL,
  speechiness tinyint NOT NULL,
  PRIMARY KEY (`ID`)
);
