
export class Gun 
{
    constructor(
        public gun_size: number,
        public n_gun: number,
        public shell_weight: number,
        public range: number,
        public SRM: number,
        public ammo: number,
        public fired: boolean = true,
        ) {}
}
export function fire(gun: Gun) 
{
    gun.ammo -= 1; 
    gun.fired = true;
}
export function check_status(gun: Gun): boolean 
{
    return gun.fired;
}
export function reset(gun: Gun) 
{
    gun.fired = false;
}
    
