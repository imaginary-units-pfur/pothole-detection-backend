import sqlite3
from os import getenv
from road_damage import *
from typing import List, Tuple


class DataBase:
    dp_path = getenv("DB_PATH") or "./db.sqlite3"

    def __init__(self):
        con = sqlite3.connect(self.dp_path)
        cur = con.cursor()
        cur.execute(
            """
        CREATE TABLE IF NOT EXISTS roadDamageTypes (
         id      INTEGER    PRIMARY KEY,
         name    TEXT NOT   NULL UNIQUE,
         UNIQUE(id, name)
        );
        """
        )

        for type in list(RoadDamageType):
            cur.execute(
                "INSERT OR IGNORE INTO roadDamageTypes(id, name) VALUES (?, ?)",
                (
                    type.value,
                    type.name,
                ),
            )
            con.commit()
        cur.execute(
            """
        CREATE TABLE IF NOT EXISTS roadDamage (
          id        INTEGER PRIMARY KEY,
          type      INTEGER NOT NULL DEFAULT (0) REFERENCES roadDamageTypes(id),
          file      TEXT    NOT NULL,
          latitude  REAL    NOT NULL,
          longitude REAL    NOT NULL,
        );
        """
        )

    def insert(self, damage_instance: RoadDamage) -> None:
        con = sqlite3.connect(self.dp_path)
        cur = con.cursor()
        cur.execute(
            """
        INSERT INTO roadDamage(type, file, latitude, longitude)
        VALUES (?, ?, ?, ?)
        """,
            (
                damage_instance.type.value,
                damage_instance.file_path,
                damage_instance.position[0],
                damage_instance.position[1],
            ),
        )
        con.commit()

    def query(self, center: tuple[float, float], radius: float) -> list[RoadDamage]:
        con = sqlite3.connect(self.dp_path)
        cur = con.cursor()
        res = cur.execute(
            """
            SELECT * FROM roadDamage
            WHERE latitude BETWEEN ? AND ?
            AND
            longitude BETWEEN ? AND ?
        """,
            (
                center[0] - radius,
                center[0] + radius,
                center[1] - radius,
                center[1] + radius,
            ),
        )
        output = []
        for row in res.fetchall():
            output.append(RoadDamage(RoadDamageType(row[1]), row[2], (row[3], row[4])))

        return output
