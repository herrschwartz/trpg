use crate::players::Player;
use rand::prelude::*;
use std::{thread, time};

#[derive(Copy, Clone)]
pub struct Enemy {
    pub name: &'static str,
    pub health: i32,
    pub dmg_phys: i32,
    pub dmg_magic: i32,
    pub armor: i32,
    pub magic_res: i32,
    pub speed: i32,
    pub crit: i8,
    pub tier: i32,

    pub entry_txt: &'static str,
    pub atk_txt: &'static str,
}

impl Enemy {
    pub fn attack(&mut self, speed_of_action: i32, player: &mut Player) {
        let mut num_atks_enemy:i32 = (speed_of_action as f32 / self.speed as f32).ceil() as i32;
        if num_atks_enemy < 1 {num_atks_enemy = 1}

        for _ in 0.. num_atks_enemy {
            let mut dmg_amt = self.dmg_phys + player.gen.gen_range(0, self.tier + 1) - player.armor;
            let dmg_magic = self.dmg_magic - player.armor_magic;
            if dmg_magic > 0 {dmg_amt += dmg_magic}
            if dmg_amt < 0 {dmg_amt = 0}

            if self.crit >= player.gen.gen_range(1, 101) {
                player.health -= dmg_amt * 2;
                println!("The {} Crits you for {} damage!! \n", self.name, dmg_amt * 2)
            } else {
                player.health -= dmg_amt;
                println!("The {} {} you for {} damage \n", self.name, self.atk_txt, dmg_amt)
            }
            thread::sleep(time::Duration::from_millis(600));
        }
    }
    pub fn new() -> Enemy {
        Enemy{name: "placeholder",health: 1,dmg_phys: 1,dmg_magic: 1,armor: 1
        ,magic_res: 1,speed: 1,crit: 1,tier: 1,atk_txt: "",entry_txt: ""}
    }

    pub fn load_t1() -> Vec<Enemy>{
        vec!(
            Enemy {
                name: "Rat",
                health: 8,
                dmg_phys: 1,
                dmg_magic: 0,
                armor: 0,
                magic_res: 0,
                speed: 2,
                crit: 3,
                tier: 1,
                atk_txt: "claws",
                entry_txt: "A rat scurries up from the darkness, it has become large and fat from chewing on the limbs of your ancestors \n"
            },
            Enemy {
                name: "Axeman",
                health: 12,
                dmg_phys: 2,
                dmg_magic: 0,
                armor: 0,
                magic_res: 0,
                speed: 3,
                crit: 5,
                tier: 1,
                atk_txt: "hacks",
                entry_txt: "A burly man with an axe stands between you and the exit. He looks determained to fell you \n"
            },
            Enemy {
                name: "Rock Worm",
                health: 10,
                dmg_phys: 1,
                dmg_magic: 0,
                armor: 1,
                magic_res: 1,
                speed: 3,
                crit: 3,
                tier: 1,
                atk_txt: "headbutts",
                entry_txt: "It's hard to see anything in this room, but you hear a low scraping sound as what appears to be a rock inches towards you \n"
            },
            Enemy {
                name: "Thief",
                health: 7,
                dmg_phys: 1,
                dmg_magic: 0,
                armor: 0,
                magic_res: 0,
                speed: 1,
                crit: 4,
                tier: 1,
                atk_txt: "stabs",
                entry_txt: "A cloaked head turns as you enter. Shocked by your pressence the shrouded figure faces you.  \n"
            }
        )
    }
    pub fn load_t2() -> Vec<Enemy>{
        vec!(
            Enemy {
                name: "Sentry Golem",
                health: 18,
                dmg_phys: 2,
                dmg_magic: 0,
                armor: 4,
                magic_res: 1,
                speed: 3,
                crit: 3,
                tier: 2,
                atk_txt: "smashes",
                entry_txt: "With thundering steps a golem charges towards you, the defence system must be on high alert \n"
            },
            Enemy {
                name: "Necromancer",
                health: 15,
                dmg_phys: 0,
                dmg_magic: 3,
                armor: 0,
                magic_res: 3,
                speed: 2,
                crit: 3,
                tier: 2,
                atk_txt: "unleashes decay at",
                entry_txt: "You spot a twisted old man rotting away the life within the sanctuary, he must be stopped \n"
            },
            Enemy {
                name: "Mercenary",
                health: 16,
                dmg_phys: 2,
                dmg_magic: 0,
                armor: 1,
                magic_res: 1,
                speed: 1,
                crit: 10,
                tier: 2,
                atk_txt: "slashes",
                entry_txt: "A well armed man stalks the halls, He seems to be looking to kill anything that moves \n"
            },
            Enemy {
                name: "Corrupted Vines",
                health: 14,
                dmg_phys: 2,
                dmg_magic: 0,
                armor: 2,
                magic_res: 2,
                speed: 2,
                crit: 5,
                tier: 2,
                atk_txt: "constrict",
                entry_txt: "Your foot is suddenly caught as you walk, the roots themselves have been turned against you \n"
            },
            Enemy {
                name: "Acolyte",
                health: 16,
                dmg_phys: 1,
                dmg_magic: 2,
                armor: 2,
                magic_res: 1,
                speed: 2,
                crit: 5,
                tier: 2,
                atk_txt: "cuts",
                entry_txt: "A women dressed in black and purple robes seemingly comes from nowhere to block you path. She draws a glowing dagger and smiles wickedly  \n"
            },
        )
    }
}


