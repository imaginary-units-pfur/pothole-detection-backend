CREATE TABLE road_damage (
  id           INTEGER  PRIMARY KEY,
  damage_type  INTEGER  NOT NULL,
  file_path    TEXT     NOT NULL,
  latitude     REAL     NOT NULL,
  longitude    REAL     NOT NULL
);
