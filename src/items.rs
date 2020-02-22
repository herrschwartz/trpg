
pub struct Buff {
    pub name: &'static str,
    pub decription: &'static str
}
impl Buff {
    pub fn add_effect(&self) {

    }
    pub fn remove_effect(&self) {

    }
}

pub struct Spell {
    pub name: &'static str,
    pub description: &'static str,
    pub speed: i32,
}

pub struct Weapon {
    pub name: &'static str,
    pub damage: i32,
    pub speed: i32,
    pub crit: i8,
    pub rank: i32,

    pub crit_txt: &'static str,
    pub atk_txt: &'static str,
}

pub struct Armor {
    pub name: &'static str,
    pub value: i32,
    pub magic_res: i32,
    pub rank: i32,
}
