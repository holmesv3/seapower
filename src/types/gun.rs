use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Gun {
    pub gun_size: f32,
    pub n_gun: f32,
    pub shell_weight: f32,
    pub range: f32,
    pub srm: f32,
    pub ammo: f32,
    fired: bool,
}

impl Gun {
    pub fn fire(&mut self) {
        self.ammo -= 1.;
        self.fired = true;
    }

    pub fn is_fired(&self) -> bool {
        self.fired
    }

    pub fn reset(&mut self) {
        self.fired = false;
    }
}
