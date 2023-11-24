from dataclasses import dataclass

from .Gun import Gun
from .Utils import Country
# def attack(ship: Ship, range: float): void 
#     let dmg = 1 / range;
#     if (ship.primary_gun)
#         gun.fire(ship.primary_gun);
#     if (ship.secondary_gun)
#         gun.fire(ship.secondary_gun);
#     if (ship.tertiary_gun)
#         gun.fire(ship.tertiary_gun);
#     ship.turn_taken = true;


# def reset_ship(ship: Ship): void 
#     if (ship.primary_gun)
#         gun.reset(ship.primary_gun);
#     if (ship.secondary_gun)
#         gun.reset(ship.secondary_gun);
#     if (ship.tertiary_gun)
#         gun.reset(ship.tertiary_gun);


# def check_status(ship: Ship): bool  
#     return ship.turn_taken;
@dataclass
class Ship:
    name: str
    ship_class: str
    country: Country
    DV: float
    max_speed: float
    status: float
    gun_status: float
    hull_status: float
    torpedo_tubesize: float
    n_tube: float
    n_torpedo: float
    primary_gun: Gun
    secondary_gun: Gun
    tertiary_gun: Gun
    catapults: float
    aircraft_on_board: float
    aircraft_in_air: float
    depth_charge: float
    anti_aircraft: float
    mines: float
    turn_taken: bool = False



class ShipsByCountry: 
    def __init__(self, ships: list[Ship]) -> None:
        
        self.usa: list[Ship] = []
        self.gb: list[Ship] = []
        self.fr: list[Ship] = []
        self.cn: list[Ship] = []
        self.rus: list[Ship] = []
        self.jpn: list[Ship] = []
        self.it: list[Ship] = []
        self.ger: list[Ship] = []
    
        for ship in ships: 
            match ship.country:
                case Country.USA:
                    self.usa.append(ship)
                case Country.GB:
                    self.gb.append(ship)
                case Country.FR:
                    self.fr.append(ship)
                case Country.CN:
                    self.cn.append(ship)
                case Country.RUS:
                    self.rus.append(ship)
                case Country.JPN:
                    self.jpn.append(ship)
                case Country.IT:
                    self.it.append(ship)
                case Country.GER:
                    self.ger.append(ship)
     
     
class Side:
    
    _all: list[Ship]
    
    @staticmethod
    def from_ships(ships: list[Ship]):
        raise NotImplementedError

    @staticmethod
    def from_ships_by_country(ships: ShipsByCountry):
        raise NotImplementedError
    
    def get_all(self) -> list[Ship]: 
        return self._all;
    
    def check_all(self) -> list[Ship]:
        """Return all ships which have actions remaning"""
        out = []
        ships = self.get_all()
        for ship in ships:
            if ship.status:
                out.append(ship)

        return out
          
  
class GoodGuys(Side):
    usa: list[Ship] = []
    gb: list[Ship] = []
    fr: list[Ship] = []
    cn: list[Ship] = []
    rus: list[Ship] = []

    @staticmethod
    def from_ships(ships: list[Ship]):
        gg = GoodGuys()
        for ship in ships: 
            match ship.country:
                case Country.USA:
                    gg.usa.append(ship)
                case Country.GB:
                    gg.gb.append(ship)
                case Country.FR:
                    gg.fr.append(ship)
                case Country.CN:
                    gg.cn.append(ship)
                case Country.RUS:
                    gg.rus.append(ship)
        gg._all = gg.usa + gg.gb + gg.fr + gg.cn + gg.rus
        return gg

    @staticmethod
    def from_ships_by_country(ships: ShipsByCountry):
        gg = GoodGuys()
        gg.usa = ships.usa
        gg.gb = ships.gb
        gg.fr = ships.fr
        gg.cn = ships.cn
        gg.rus = ships.rus
        gg._all = gg.usa + gg.gb + gg.fr + gg.cn + gg.rus
        return gg
        

class BadGuys(Side):
    jpn: list[Ship] = []
    it: list[Ship] = []
    ger: list[Ship] = []

    @staticmethod
    def from_ships(ships: list[Ship]):
        bg = BadGuys()
        for ship in ships: 
            match ship.country:
                case Country.JPN:
                    bg.jpn.append(ship)
                case Country.IT:
                    bg.it.append(ship)
                case Country.GER:
                    bg.ger.append(ship)
        bg._all = bg.jpn + bg.it + bg.ger
        return bg

    @staticmethod
    def from_ships_by_country(ships: ShipsByCountry):
        bg = BadGuys()
        bg.jpn = ships.jpn
        bg.it = ships.it
        bg.ger = ships.ger
        bg._all = bg.jpn + bg.it + bg.ger
        return bg
    

class ShipsBySide:
    good_guys: GoodGuys
    bad_guys: BadGuys

    @staticmethod
    def from_ships(ships: list[Ship]):
        sbs = ShipsBySide()
        sbs.good_guys = GoodGuys(ships)
        sbs.bad_guys = BadGuys(ships)
        return sbs

    @staticmethod
    def from_ships_by_country(ships: ShipsByCountry):
        sbs = ShipsBySide()
        sbs.good_guys = GoodGuys(ships)
        sbs.bad_guys = BadGuys(ships)
        return sbs
    