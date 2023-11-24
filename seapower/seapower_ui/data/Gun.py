from dataclasses import dataclass

@dataclass
class Gun: 
    gun_size: float
    n_gun: float
    shell_weight: float
    range: float
    SRM: float
    ammo: float
    fired: bool = True

    def fire(self): 
        self.ammo -= 1
        self.fired = True

    def check_status(self) -> bool:
        return self.fired

    def reset(self): 
        self.fired = False
