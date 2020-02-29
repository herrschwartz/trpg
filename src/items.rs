
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
#[derive(Clone)]
pub struct Spell {
    pub name: &'static str,
    pub description: &'static str,
    pub speed: i32,
    pub damage: i32,
    pub kind: &'static str,

    pub atk_txt: &'static str,
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

impl Spell {
    pub fn load_t1_spells() -> Vec<Spell> {
        vec!{
            Spell {
                name: "arcane bolt", 
                description: "A basic bolt of magic energy", 
                speed: 1, 
                damage: 2, 
                kind: "arcane",
                atk_txt: "Your hands fume with perverse energies, they coalese into a dense bead"
            },
            Spell {
                name: "fire ball", 
                description: "A normal fireball", 
                speed: 1, 
                damage: 6, 
                kind: "fire",
                atk_txt: "Your hands glow and a small ball of bright heat forms"
            },
            Spell {
                name: "fire blast", 
                description: "Slower to cast than a fireball, but sure to get the job done", 
                speed: 2, 
                damage: 8, 
                kind: "fire",
                atk_txt: "You channel all of your strength into your hands. Releasing the fire within"
            },
            Spell {
                name: "static shock", 
                description: "A shock maybe enough to stop a heart", 
                speed: 1, 
                damage: 0, 
                kind: "lightning",
                atk_txt: "Hairs raise on your arms, you throw them forward and let the sparks fly."
            },
        }
    }
}