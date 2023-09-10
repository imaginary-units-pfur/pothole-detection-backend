CREATE TABLE IF NOT EXISTS road_damage (
  id           INTEGER  PRIMARY KEY NOT NULL AUTO_INCREMENT,
  damage_type  INTEGER  NOT NULL,
  file_path    TEXT     NOT NULL,
  latitude     REAL     NOT NULL,
  longitude    REAL     NOT NULL
);
