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
    pub fn attack(&mut self, num_atks_enemy: i32, player: &mut Player) {
        for _ in 0.. num_atks_enemy {
            let mut dmg_amt = self.dmg_phys + player.gen.gen_range(0, self.tier + 1) - player.armor.value;
            let dmg_magic = self.dmg_magic - player.armor.magic_res;
            if dmg_magic > 0 {dmg_amt += dmg_magic}
            if dmg_amt < 0 {dmg_amt = 0}

            if self.crit >= player.gen.gen_range(1, 101) {
                player.health -= dmg_amt * 2;
                println!("The {} crits you for {} damage! ow \nYou have {} health", self.name, dmg_amt * 2, player.health)
            } else {
                player.health -= dmg_amt;
                println!("The {} {} you for {} damage \nYou have {} health", self.name, self.atk_txt, dmg_amt, player.health)
            }
            thread::sleep(time::Duration::from_millis(600));
        }
    }
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
            entry_txt: "It's hard to see anything in this room, but you hear a low scraping sound as what looks like a rock inches toward you \n"
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