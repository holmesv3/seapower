from enum import StrEnum

class GameScale(StrEnum):
    SM='SM'
    MD='MD'
    LG='LG'


class Country(StrEnum):
    USA = "United States"
    GB = "Great Britain"
    FR = "France"
    CN = "Canada"
    RUS = "Russia"
    JPN = "Japan"
    IT = "Italy"
    GER = "Germany"

    
class ShipClass(StrEnum):
    DESTROYER = "Destroyer"
    BATTLESHIP = "Battleship"
    CRUISER = "Cruiser"
    SUBMARINE = "Submarine"
    CARRIER = "Carrier"
    