
from enum import Enum


class GunType(Enum):
    Primary = "Primary"
    Secondary = "Secondary"
    Tertiary = "Tertiary"


class ShipGun:
    """
    gun_type

    gun_size

    n_gun

    shell_weight

    range

    SRM

    ammo
    """

    def __init__(
        self,
        gun_type: GunType,
        gun_size: float,
        n_gun: int,
        shell_weight: float,
        range: float,
        SRM: float,
        ammo: int,
    ):
        # I
        self.gun_type = gun_type
        # J
        self.gun_size = gun_size
        # K
        self.n_gun = n_gun
        # L
        self.shell_weight = shell_weight
        # M
        self.range = range
        # N
        self.SRM = SRM
        # O
        self.ammo = ammo


class G_20_5_38DP(ShipGun):
    def __init__(
        self,
        gun_type: GunType,
        gun_size: float = 5,
        n_gun: int = 20,
        shell_weight: float = 55,
        range: float = 17500,
        SRM: float = 75,
        ammo: int = 9000,
    ):
        super().__init__(gun_type, gun_size, n_gun, shell_weight, range, SRM, ammo)


class G_16_5_38DP(ShipGun):
    def __init__(
        self,
        gun_type: GunType,
        gun_size: float = 5,
        n_gun: int = 20,
        shell_weight: float = 55,
        range: float = 17500,
        SRM: float = 75,
        ammo: int = 7200,
    ):
        super().__init__(gun_type, gun_size, n_gun, shell_weight, range, SRM, ammo)


class G_9_16_45R(ShipGun):
    def __init__(
        self,
        gun_type: GunType,
        gun_size: float = 16,
        n_gun: int = 9,
        shell_weight: float = 2700,
        range: float = 40500,
        SRM: float = 10,
        ammo: int = 1080,
    ):
        super().__init__(gun_type, gun_size, n_gun, shell_weight, range, SRM, ammo)


class G_12_16_50(ShipGun):
    def __init__(
        self,
        gun_type: GunType,
        gun_size: float = 16,
        n_gun: int = 12,
        shell_weight: float = 2100,
        range: float = 44500,
        SRM: float = 7.5,
        ammo: int = 1080,
    ):
        super().__init__(gun_type, gun_size, n_gun, shell_weight, range, SRM, ammo)
