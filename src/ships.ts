

enum ShipClass 
{
    DESTROYER = "Destroyer",
    BATTLESHIP = "Battleship",
    CRUISER = "Cruiser",
    SUBMARINE = "Submarine",
    CARRIER = "Carrier",
}

enum Country
{
    USA = "United States",
    GB = "Great Britain",
    FR = "France",
    CN = "Canada",
    RUS = "Russia",
    JPN = "Japan",
    IT = "Italy",
    GER = "Germany",
}

enum GunType
{
    Primary = "Primary",
    Secondary = "Secondary",
    Tertiary = "Tertiary",
}

class ShipGun 
{
    gun_type: GunType;
    gun_size: number;
    n_gun: number;
    shell_weight: number;
    range: number;
    SRM: number;
    ammo: number;

    constructor
    (
        gun_type: GunType, 
        gun_size: number,
        n_gun: number,
        shell_weight: number,
        range: number,
        SRM: number,
        ammo: number
    ) 
    {
        this.gun_type = gun_type;
        this.gun_size = gun_size;
        this.n_gun = n_gun;
        this.shell_weight = shell_weight;
        this.range = range;
        this.SRM = SRM;
        this.ammo = ammo;
    }
}

class Dice {} // TODO

class Ship 
{
    name: String;
    shiClass: ShipClass;
    country: Country;
    DV: number; 
    max_speed: number;
    primary_gun: ShipGun;
    status: number;
    gun_status: number;
    hull_status: number;
    torpedo_tubesize: number;
    n_tube: number;
    n_torpedo: number;
    secondary_gun: ShipGun;
    tertiary_gun: ShipGun;
    catapults: number;
    aircraft_on_board: number;
    aircraft_in_air: number;
    depth_charge=null; 
    anti_aircraft: Dice;
    mines=null; 
    constructor(
        name: String = 'None',
        shiClass: ShipClass,
        country: Country,
        DV: number,
        max_speed: number,
        primary_gun: ShipGun,
        status: number,
        gun_status: number,
        hull_status: number,
        torpedo_tubesize: number,
        n_tube: number,
        n_torpedo: number,
        secondary_gun: ShipGun,
        tertiary_gun: ShipGun,
        catapults: number,
        aircraft_on_board: number,
        aircraft_in_air: number,
        depth_charge=null,
        anti_aircraft: Dice,
        mines=null,
    ) {
        this.name = name;
        this.shiClass = shiClass;
        this.country = country;
        this.DV = DV;
        this.max_speed = max_speed;
        this.primary_gun = primary_gun;
        this.status = status;
        this.gun_status = gun_status;
        this.hull_status = hull_status;
        this.torpedo_tubesize = torpedo_tubesize;
        this.n_tube = n_tube;
        this.n_torpedo = n_torpedo;
        this.secondary_gun = secondary_gun;
        this.tertiary_gun = tertiary_gun;
        this.catapults = catapults;
        this.aircraft_on_board = aircraft_on_board;
        this.aircraft_in_air = aircraft_in_air;
        this.depth_charge = depth_charge;
        this.anti_aircraft = anti_aircraft;
        this.mines = mines;
    }
    

}
