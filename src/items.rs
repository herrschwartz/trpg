use crate::players::Player;
use crate::enemys::Enemy;

#[derive(Clone)]
pub struct Blessing {
    pub name: &'static str,
    pub description: &'static str,
    pub speed: i32,
    pub retaliation: bool,
    pub combat_only: bool,

    pub invoke_txt: &'static str,
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

#[derive(Clone)]
pub struct Weapon {
    pub name: &'static str,
    pub damage: i32,
    pub speed: i32,
    pub crit: i8,
    pub rank: i32,

    pub crit_txt: &'static str,
    pub atk_txt: &'static str,
}


impl Spell {
    pub fn load_t1_spells() -> Vec<Spell> {
        vec!{
            Spell {
                name: "Arcane Bolt", 
                description: "A basic bolt of magic energy", 
                speed: 1, 
                damage: 2, 
                kind: "arcane",
                atk_txt: "Your hands fume with perverse energies, they coalese into a dense bead"
            },
            Spell {
                name: "Fire Ball", 
                description: "A normal fireball", 
                speed: 1, 
                damage: 6, 
                kind: "fire",
                atk_txt: "Your hands glow and a small ball of bright heat forms"
            },
            Spell {
                name: "Fire Blast", 
                description: "Slower to cast than a fireball, but sure to get the job done", 
                speed: 2, 
                damage: 9, 
                kind: "fire",
                atk_txt: "You channel all of your strength into your hands. Releasing the fire within"
            },
            Spell {
                name: "Static Shock", 
                description: "A shock maybe enough to stop a heart", 
                speed: 1, 
                damage: 0, 
                kind: "lightning",
                atk_txt: "Hairs raise on your arms, you throw them forward and let the sparks fly."
            },
            Spell {
                name: "Ball Lightning", 
                description: "A dense packet of electricity, slow to cast", 
                speed: 2, 
                damage: 2, 
                kind: "lightning",
                atk_txt: "You cup both of your hands together, concetrating as much electric potention as you can between them"
            },
        }
    }
}

impl Weapon {
    pub fn load_t1_weapons() -> Vec<Weapon> {
        vec![
            Weapon {
                name: "Dagger",
                damage: 1,
                speed: 1,
                crit: 4,
                rank: 1,
                crit_txt: "eviscerate",
                atk_txt: "stab",
            },
            Weapon {
                name: "Longsword",
                damage: 2,
                speed: 2,
                crit: 3,
                rank: 1,
                crit_txt: "imaple",
                atk_txt: "cut",
            },
            Weapon {
                name: "Spiked Mace",
                damage: 1,
                speed: 2,
                crit: 8,
                rank: 1,
                crit_txt: "imaple",
                atk_txt: "cut",
            },
            Weapon {
                name: "Claymore",
                damage: 5,
                speed: 3,
                crit: 3,
                rank: 1,
                crit_txt: "destroy",
                atk_txt: "slash",
            }
        ]
    }
}

impl Blessing {
    pub fn invoke_effect(&self, player: &mut Player, enemy: &mut Enemy) {
        match self.name {
            "Heal" => player.heal(2 + player.devotion / 2),
            "Holy Strength" => player.strength += 1,
            "Supress" => enemy.dmg_phys -= 1,
            "Holy Wrath" => {
                enemy.health -= 4;
                player.heal(2);
                println!("The burst of holy energy hits the {} for 4 damage", enemy.name);
            },
            "Protection" => {
                player.armor += 1;
                player.armor_magic += 1;
                println!("Your armor and magic resistance increase!");
            }
            _ => panic!("No cast effect for {}", self.name)
        }
    }
    pub fn loat_t1_blessings() -> Vec<Blessing> {
        vec![
            Blessing {
                name: "Heal",
                description: "A basic heal that scales with devotion",
                speed: 1,
                retaliation: true,
                combat_only: false,
                invoke_txt: "You bask in holy light, restoring your vitality"
            },
            Blessing {
                name: "Heal",
                description: "A basic heal that scales with devotion",
                speed: 1,
                retaliation: true,
                combat_only: false,
                invoke_txt: "You bask in holy light, restoring your vitality"
            },
            Blessing {
                name: "Holy Strength",
                description: "Increases your strength slightly for this combat",
                speed: 1,
                retaliation: false,
                combat_only: true,
                invoke_txt: "You kneel and speak a word of power, your strength increases"
            },
            Blessing {
                name: "Supress",
                description: "Slightly reduces your enemy's damage for this combat",
                speed: 1,
                retaliation: false,
                combat_only: true,
                invoke_txt: "You clasp you hands together and conjure glowing shackles around your foe"
            },
            Blessing {
                name: "Holy Wrath",
                description: "Strikes your foe with holy light while healing you",
                speed: 2,
                retaliation: true,
                combat_only: true,
                invoke_txt: "You reach your hand to the sky pulling holy energy into your body. Pushing your hands foward you unleash it upon your enemy."
            },
            Blessing {
                name: "Protection",
                description: "Shrounds you in holy energy, increasing your armor",
                speed: 1,
                retaliation: true,
                combat_only: true,
                invoke_txt: "You pull the surrounding energy into yourself, shielding you from evil."
            }
        ]
    }
}

pub fn remove_effect(player: &mut Player, blessing_name: &'static str) {
    match blessing_name {
        "Holy Strength" => player.strength -= 1,
        "Protection" => {
            player.armor -= 1;
            player.armor_magic -= 1;
        }
        _ => panic!("Failed to remove {}", blessing_name)
    }
}