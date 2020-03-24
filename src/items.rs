use crate::players::Player;
use crate::enemys::Enemy;

#[derive(Clone)]
pub struct Blessing {
    pub name: &'static str,
    pub description: &'static str,
    pub speed: i32,
    pub retaliation: bool,
    pub combat_only: bool,
    pub active_effect: bool,

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
                damage: 5, 
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

    pub fn load_t2_spells() -> Vec<Spell> {
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
                name: "Arcane Missle", 
                description: "A basic bolt of magic energy", 
                speed: 1, 
                damage: 4, 
                kind: "arcane",
                atk_txt: "Your hands fume with perverse energies, they coalese into a refined point"
            },
            Spell {
                name: "Arcane Missle", 
                description: "A basic bolt of magic energy", 
                speed: 1, 
                damage: 4, 
                kind: "arcane",
                atk_txt: "Your hands fume with perverse energies, they coalese into a refined point"
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
                name: "Fire Spear", 
                description: "A quick and piercing fire spell", 
                speed: 1, 
                damage: 7, 
                kind: "fire",
                atk_txt: "You channel all of your strength into your hands. Releasing the fire within"
            },
            Spell {
                name: "Thunder Bolt", 
                description: "A bolt of dense plasma", 
                speed: 1, 
                damage: 2, 
                kind: "lightning",
                atk_txt: "You throw your arms forward, lightning follows and instantly hits your enemy."
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
                crit: 5,
                rank: 1,
                crit_txt: "imaple",
                atk_txt: "cut",
            },
            Weapon {
                name: "Spiked Mace",
                damage: 1,
                speed: 2,
                crit: 15,
                rank: 1,
                crit_txt: "bludgeon",
                atk_txt: "bash",
            },
            Weapon {
                name: "Claymore",
                damage: 5,
                speed: 3,
                crit: 4,
                rank: 1,
                crit_txt: "destroy",
                atk_txt: "slash",
            }
        ]
    }
    pub fn load_t2_weapons() -> Vec<Weapon> {
        vec![
            Weapon {
                name: "Rapier",
                damage: 1,
                speed: 1,
                crit: 8,
                rank: 1,
                crit_txt: "pierce",
                atk_txt: "stab",
            },
            Weapon {
                name: "Broadsword",
                damage: 3,
                speed: 2,
                crit: 5,
                rank: 1,
                crit_txt: "imaple",
                atk_txt: "cut",
            },
            Weapon {
                name: "War Hammer",
                damage: 2,
                speed: 2,
                crit: 14,
                rank: 1,
                crit_txt: "bludgeon",
                atk_txt: "bash",
            },
            Weapon {
                name: "Greatsword",
                damage: 5,
                speed: 3,
                crit: 7,
                rank: 1,
                crit_txt: "bisect",
                atk_txt: "slash",
            }
        ]
    }
}

impl Blessing {
    pub fn invoke_effect(&self, player: &mut Player, enemy: &mut Enemy) {
        match self.name {
            "Heal" => player.heal(2 + player.devotion / 2),
            "Greater Heal" => player.heal(3 + player.devotion),
            "Holy Strength" => player.strength += 1,
            "Divine Strength" => player.strength += 2,
            "Supress" => enemy.dmg_phys -= 1,
            "Holy Wrath" => {
                enemy.health -= 4;
                player.heal(2);
                println!("The burst of holy energy hits the {} for 4 damage", enemy.name);
            },
            "Divine Wrath" => {
                enemy.health -= 6;
                player.heal(4);
                println!("The burst of holy energy hits the {} for 6 damage", enemy.name);
            },
            "Protection" => {
                player.armor += 1;
                player.armor_magic += 1;
                println!("Your armor and magic resistance increase!");
            },
            "Find Weakness" => {
                enemy.armor -= 1;
                enemy.magic_res -= 1;
            }
            "Sacrafice" => {
                let dmg = 2 + player.strength/2 + player.int/2 + player.devotion/2 + player.resolve/2;
                enemy.health -= dmg;
                println!("{} is hit for {} damage", enemy.name, dmg);
                player.health -= player.health/2;
                println!("You take {} damage", player.health/2);
            }
            _ => panic!("No cast effect for {}", self.name)
        }
    }
    pub fn load_t1_blessings() -> Vec<Blessing> {
        vec![
            Blessing {
                name: "Heal",
                description: "A basic heal that scales with devotion",
                speed: 1,
                retaliation: true,
                combat_only: false,
                active_effect: false,
                invoke_txt: "You bask in holy light, restoring your vitality"
            },
            Blessing {
                name: "Heal",
                description: "A basic heal that scales with devotion",
                speed: 1,
                retaliation: true,
                combat_only: false,
                active_effect: false,
                invoke_txt: "You bask in holy light, restoring your vitality"
            },
            Blessing {
                name: "Holy Strength",
                description: "Increases your strength slightly for this combat",
                speed: 1,
                retaliation: false,
                combat_only: true,
                active_effect: true,
                invoke_txt: "You kneel and speak a word of power, your strength increases"
            },
            Blessing {
                name: "Supress",
                description: "Slightly reduces your enemy's damage for this combat",
                speed: 1,
                retaliation: false,
                combat_only: true,
                active_effect: false,
                invoke_txt: "You clasp you hands together and conjure glowing shackles around your foe"
            },
            Blessing {
                name: "Holy Wrath",
                description: "Strikes your foe with holy light while healing you",
                speed: 2,
                retaliation: true,
                combat_only: true,
                active_effect: false,
                invoke_txt: "You reach your hand to the sky pulling holy energy into your body. Pushing your hands foward you unleash it upon your enemy."
            },
            Blessing {
                name: "Protection",
                description: "Shrounds you in holy energy, increasing your armor",
                speed: 1,
                retaliation: true,
                combat_only: true,
                active_effect: false,
                invoke_txt: "You pull the surrounding energy into yourself, shielding you from evil."
            }
        ]
    }
    pub fn load_t2_blessings() -> Vec<Blessing> {
        vec! [
            Blessing {
                name: "Heal",
                description: "A basic heal that scales with devotion",
                speed: 1,
                retaliation: true,
                combat_only: false,
                active_effect: false,
                invoke_txt: "You bask in holy light, restoring your vitality"
            },
            Blessing {
                name: "Greater Heal",
                description: "A heal that scales with devotion, slower than Heal",
                speed: 2,
                retaliation: true,
                combat_only: false,
                active_effect: false,
                invoke_txt: "You bask in a wave of Holy Light, restoring your vitality"
            },
            Blessing {
                name: "Divine Strength",
                description: "Increases your strength for this combat",
                speed: 1,
                retaliation: false,
                combat_only: true,
                active_effect: true,
                invoke_txt: "You kneel and speak a word of greater power,\n ...your strength increases"
            },
            Blessing {
                name: "Supress",
                description: "Slightly reduces your enemy's damage for this combat",
                speed: 1,
                retaliation: false,
                combat_only: true,
                active_effect: false,
                invoke_txt: "You clasp you hands together and conjure glowing shackles around your foe"
            },
            Blessing {
                name: "Holy Wrath",
                description: "Strikes your foe with holy light while healing you",
                speed: 2,
                retaliation: true,
                combat_only: true,
                active_effect: false,
                invoke_txt: "You reach your hand to the sky pulling holy energy into your body. Pushing your hands foward you unleash it upon your enemy."
            },
            Blessing {
                name: "Protection",
                description: "Shrounds you in holy energy, increasing your armor",
                speed: 1,
                retaliation: true,
                combat_only: true,
                active_effect: false,
                invoke_txt: "You pull the surrounding energy into yourself, shielding you from evil."
            },
            Blessing {
                name: "Divine Wrath",
                description: "Strikes your foe with holy light while healing you",
                speed: 2,
                retaliation: true,
                combat_only: true,
                active_effect: false,
                invoke_txt: "You reach your hand to the sky pulling holy energy into your body. Pushing your hands foward you unleash it upon your enemy."
            },
            Blessing {
                name: "Find Weakness",
                description: "Lowers your enemy's armor",
                speed: 1,
                retaliation: false,
                combat_only: true,
                active_effect: false,
                invoke_txt: "Through your prays you are granted insight"
            },
            Blessing {
                name: "Sacrafice",
                description: "Sacrafices your health to do great damage to you enemy",
                speed: 1,
                retaliation: true,
                combat_only: true,
                active_effect: false,
                invoke_txt: "You stretch arms out wide, pooling all of the surrounding energy within you. 
                ...Surrounded by Holy Light, you slam your body into you enemy with all of your strengh...
                        is this it?"
            },
        ]
    }
}

pub fn remove_effect(player: &mut Player, blessing_name: &'static str) {
    match blessing_name {
        "Holy Strength" => player.strength -= 1,
        "Divine Strength" => player.strength -= 2,
        "Protection" => {
            player.armor -= 1;
            player.armor_magic -= 1;
        }
        _ => panic!("Failed to remove {}", blessing_name)
    }
}