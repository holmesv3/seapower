from ships.define import Ship, ShipClass
from stuff import ShipClass, Country, ColoredDice, LeteredDice
from guns import *

### UNITED STATES
_ship_class = ShipClass.BATTLESHIP


class Montana(Ship):
    def __init__(self):
        name = 'Montana'
        ship_class = _ship_class
        country = Country.USA
        DV = 0.5
        max_speed = 12
        primary_gun = ShipGun(
            gun_type=GunType.Primary,
            n_gun=12, 
            gun_size=16, 
            shell_weight=2700, 
            range=42500,
            SRM=10,
            ammo=1440, 
        )
        secondary_gun = ShipGun(
            gun_type=GunType.Secondary,
            n_gun=20, 
            gun_size=5, 
            shell_weight=70, 
            range=26000,
            SRM=150,
            ammo=18000, 
        )
        aircraft_on_board=4
        catapults=2
        anti_aircraft = ColoredDice.Blue
        
        super().__init__(
            name=name,
            ship_class=ship_class,
            country=country,
            DV=DV,
            max_speed=max_speed,
            primary_gun=primary_gun,    
            secondary_gun=secondary_gun,
            aircraft_on_board=aircraft_on_board,
            catapults=catapults,
            anti_aircraft=anti_aircraft,
        )

class Iowa(Ship):
    def __init__(self):
        name = 'Iowa'
        ship_class = _ship_class
        country = Country.USA
        DV = 1
        max_speed = 11.8
        primary_gun = ShipGun(
            gun_type=GunType.Primary,
            n_gun=9, 
            gun_size=16, 
            shell_weight=2700, 
            range=42500,
            SRM=10,
            ammo=1080, 
        )
        secondary_gun = G_20_5_38DP(gun_type=GunType.Secondary)
        aircraft_on_board=4
        catapults=2
        anti_aircraft = ColoredDice.Blue
        
        super().__init__(
            name=name,
            ship_class=ship_class,
            country=country,
            DV=DV,
            max_speed=max_speed,
            primary_gun=primary_gun,    
            secondary_gun=secondary_gun,
            aircraft_on_board=aircraft_on_board,
            catapults=catapults,
            anti_aircraft=anti_aircraft,
        )

class Indiana(Ship):
    def __init__(self):
        name = 'Indiana'
        ship_class = _ship_class
        country = Country.USA
        DV = 2
        max_speed = 8.2
        primary_gun = G_9_16_45R(gun_type=GunType.Primary)
        secondary_gun = G_20_5_38DP(gun_type=GunType.Secondary)
        aircraft_on_board=4
        catapults=2
        anti_aircraft = ColoredDice.Blue
        
        super().__init__(
            name=name,
            ship_class=ship_class,
            country=country,
            DV=DV,
            max_speed=max_speed,
            primary_gun=primary_gun,    
            secondary_gun=secondary_gun,
            aircraft_on_board=aircraft_on_board,
            catapults=catapults,
            anti_aircraft=anti_aircraft,
        )

# South_Dakota = Ship(
#     name="South Dakota",
#     ship_class=_ship_class,
#     country=Country.USA,
# )

# North_Carolina = Ship(
#     name="North Carolina",
#     ship_class=_ship_class,
#     country=Country.USA,
# )
