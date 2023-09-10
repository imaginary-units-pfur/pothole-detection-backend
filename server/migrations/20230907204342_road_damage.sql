CREATE TABLE IF NOT EXISTS road_damage (
  id           BIGINT  UNSIGNED  PRIMARY KEY NOT NULL AUTO_INCREMENT,
  damage_type  INTEGER UNSIGNED  NOT NULL,
  file_path    TEXT              NOT NULL,
  latitude     REAL              NOT NULL,
  longitude    REAL              NOT NULL
);
