from enum import Enum
from aenum import MultiValueEnum

class Dice:
    ...


class ColoredDice(Dice, MultiValueEnum):
    Blue = 1, 2, 3
    Orange = 4, 5
    Red = 6


class LeteredDice(Dice, Enum):
    B = 1
    C = 2
    D = 3
    P = 4
    S = 5
    T = 6


class ShipClass(Enum):
    DESTROYER = "Destroyer"
    BATTLESHIP = "Battleship"
    CRUISER = "Cruiser"
    SUBMARINE = "Submarine"
    CARRIER = "Carrier"


class Country(Enum):
    USA = "United States"
    GB = "Great Britain"
    FR = "France"
    CN = "Canada"
    RUS = "Russia"
    JPN = "Japan"
    IT = "Italy"
    GER = "Germany"


class ShipAircraft:
    # VT
    vt: int
    # VA
    va: int
    # VF
    vf: int
    # S
    s: int
