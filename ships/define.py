from stuff import *
from guns import *
from stuff import Country, Dice, ShipAircraft, ShipClass

class Ship:
    def __init__(
        self,
        name: str,
        ship_class: ShipClass,
        country: Country,
        DV: float,
        max_speed: float,
        primary_gun: ShipGun,
        status: float = 100,  # Default starting value, could be modified I s'pose
        gun_status: float = 100,  # Default starting value, could be modified I s'pose
        hull_status: float = 100,  # Default starting value, could be modified I s'pose
        torpedo_tubesize: float = None,
        n_tube: int = None,
        n_torpedo: int = None,
        secondary_gun: ShipGun = None,
        tertiary_gun: ShipGun = None,
        catapults: int = 0,
        aircraft_on_board: ShipAircraft = None,
        aircraft_in_air: ShipAircraft = None,
        depth_charge=None,
        anti_aircraft: Dice = None,
        mines=None,
    ):
        # A
        self.name: str = name
        # B
        self.ship_class: ShipClass = ship_class
        # C
        self.country: Country = country
        # D
        self.DV: str = DV
        # E
        self.max_speed: float = max_speed
        # F
        self.torpedo_tubesize: float = torpedo_tubesize
        # G
        self.n_tube: int = n_tube
        # H
        self.n_torpedo: int = n_torpedo
        # I - O
        self.primary_gun: ShipGun = primary_gun
        # P - V
        self.secondary_gun: ShipGun = secondary_gun
        # W - CC
        self.tertiary_gun: ShipGun = tertiary_gun
        # DD
        self.catapults: int = catapults
        # XXB
        self.aircraft_on_board: int = (
            aircraft_on_board  # ShipAircraft = aircraft_on_board
        )
        # XXA
        self.aircraft_in_air: int = aircraft_in_air  # ShipAircraft = aircraft_in_air
        # EE
        self.gun_status: float = gun_status
        # FF
        self.hull_status: float = hull_status
        # GG
        self.depth_charge: str = depth_charge
        # HH
        self.anti_aircraft = anti_aircraft
        # II
        self.status: float = status
        # JJ
        self.mines = mines


