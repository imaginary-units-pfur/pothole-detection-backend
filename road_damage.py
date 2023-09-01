from pathlib import Path
from enum import Enum
from typing import Tuple


class RoadDamageType(Enum):
    NONE = 0
    POTHOLE = 2
    CRACK = 4
    PATCH = 8


class RoadDamage:
    def __init__(
        self, type: RoadDamageType, file_path: Path, position: tuple[float, float]
    ):
        self.type = type
        self.file_path = file_path
        self.position = position
