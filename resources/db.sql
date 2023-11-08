CREATE DATABASE IF NOT EXISTS SplProject;

USE SplProject;

CREATE TABLE IF NOT EXISTS Track (
  ID int NOT NULL AUTO_INCREMENT,
  track_name varchar(250) DEFAULT NULL,
  artist_name varchar(250) DEFAULT NULL,
  artist_count tinyint DEFAULT NULL,
  release_year smallint DEFAULT NULL,
  release_month tinyint DEFAULT NULL,
  release_day tinyint DEFAULT NULL,
  streams bigint DEFAULT NULL,
  bpm smallint DEFAULT NULL,
  danceability tinyint DEFAULT NULL,
  valence tinyint DEFAULT NULL,
  energy tinyint DEFAULT NULL,
  acousticness tinyint DEFAULT NULL,
  instrumentalness tinyint DEFAULT NULL,
  liveness tinyint DEFAULT NULL,
  speechiness tinyint DEFAULT NULL,
  PRIMARY KEY (`ID`)
);
