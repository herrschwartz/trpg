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
    pub fn attack(&mut self, player: &mut Player) {
        let mut dmg_amt = self.dmg_phys + player.gen.gen_range(0, self.tier + 1) - player.armor;
        let dmg_magic = self.dmg_magic - player.armor_magic;
        if dmg_magic > 0 {dmg_amt += dmg_magic}
        if dmg_amt < 0 {dmg_amt = 0}

        if self.crit >= player.gen.gen_range(1, 101) {
            player.health -= dmg_amt * 2;
            print!("The {} ", self.name);
            player.print_red("Crits ");
            println!("you for {} damage!! \n", dmg_amt * 2);
        } else {
            player.health -= dmg_amt;
            println!("The {} {} you for {} damage \n", self.name, self.atk_txt, dmg_amt)
        }
        thread::sleep(time::Duration::from_millis(600));

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
                name: "glowbug",
                health: 10,
                dmg_phys: 0,
                dmg_magic: 1,
                armor: 0,
                magic_res: 0,
                speed: 2,
                crit: 5,
                tier: 1,
                atk_txt: "dusts",
                entry_txt: "You hear a buzzing in the air, as it grows louder you begin to see a light. as you stare you notice it's coming right for your head! \n"
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
                health: 17,
                dmg_phys: 0,
                dmg_magic: 4,
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
                health: 17,
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
                health: 15,
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
                health: 17,
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
    pub fn load_t3() -> Vec<Enemy>{
        vec!(
            Enemy {
                name: "Shadow Image",
                health: 26,
                dmg_phys: 2,
                dmg_magic: 2,
                armor: 3,
                magic_res: 2,
                speed: 3,
                crit: 4,
                tier: 3,
                atk_txt: "chops",
                entry_txt: "An image of the evil you pursue appears in front of you, sent to slow you but it does not fool your keen senses \n"
            },
            Enemy {
                name: "Shadow fiend",
                health: 24,
                dmg_phys: 1,
                dmg_magic: 1,
                armor: 3,
                magic_res: 2,
                speed: 1,
                crit: 4,
                tier: 3,
                atk_txt: "bites",
                entry_txt: "A doglike creature springs out of the darkness as you descend the ramp, a sign of the spreading corruption \n"
            },
            Enemy {
                name: "Crazed Ent",
                health: 25,
                dmg_phys: 3,
                dmg_magic: 0,
                armor: 3,
                magic_res: 3,
                speed: 2,
                crit: 3,
                tier: 3,
                atk_txt: "sweeps",
                entry_txt: "The ground shakes and you see an ent charging up the ramp towards you, somthing is wrong though... \n"
            },
            Enemy {
                name: "Living Branches",
                health: 26,
                dmg_phys: 2,
                dmg_magic: 0,
                armor: 4,
                magic_res: 1,
                speed: 1,
                crit: 5,
                tier: 3,
                atk_txt: "grasp",
                entry_txt: "The path is thick with brances, in fact it seems to be getting thicker every step \n"
            },
            Enemy {
                name: "Enchanted Wall",
                health: 6,
                dmg_phys: 0,
                dmg_magic: 2,
                armor: 12,
                magic_res: 1,
                speed: 3,
                crit: 0,
                tier: 3,
                atk_txt: "reflects your hit back at you",
                entry_txt: "Suddenly a wall springs up from the ground, there is no way around \n"
            },
        )
    }
    pub fn final_boss_phase_2() -> Enemy{
        Enemy {
            name: "Dark Ent",
            health: 76,
            dmg_phys: 3,
            dmg_magic: 2,
            armor: 3,
            magic_res: 2,
            speed: 2,
            crit: 5,
            tier: 3,
            atk_txt: "punches",
            entry_txt: "
               laughing echos deep and slow, rumbling off of the walls.
               \"You think that you have won?\" the dark figure's voice rasps
               He pludges his hand down onto a massive root. Drainging power from it.
               His body twists and transforms before your eyes. branches a room spring from his skin. \n"
        }
    }
}

pub fn sanctum_guardian(turn_counter: i32, mut enemy: &mut Enemy, player: &Player) {
    if turn_counter % 3 == 0 {
        if enemy.magic_res == 0 {
            player.print_purple("The Guardians's body and limbs glow with magic runes\n");
            enemy.magic_res = 10;
            enemy.armor = 1;
            enemy.dmg_phys = 0;
            enemy.dmg_magic = 3;
        } else {
            player.print_purple("The Guardian's outer shell hardens, the runes stop glowing\n");
            enemy.magic_res = 0;
            enemy.armor = 10;
            enemy.dmg_phys = 3;
            enemy.dmg_magic = 0;
        }
    }
}

pub fn dark_ent(turn_counter: i32, mut player: &mut Player) {
    if turn_counter % 2 == 0 {
        let swap1: &str;
        let swap2: &str;
        match player.gen.gen_range(0,3) {
            0 => {
                let tmp = player.strength;
                swap1 = "Strength";
                match player.gen.gen_range(0,2){
                    0 => {
                        swap2 = "Intellect";
                        player.strength = player.int;
                        player.int = tmp;
                    }
                    1 => {
                        swap2 = "Devotion";
                        player.strength = player.devotion;
                        player.devotion = tmp;
                    }
                    _ => panic!("swap fail")
                } 
            }
            1 => {
                let tmp = player.int;
                swap1 = "Intellect";
                match player.gen.gen_range(0,2){
                    0 => {
                        swap2 = "Strength";
                        player.int = player.strength;
                        player.strength = tmp;
                    }
                    1 => {
                        swap2 = "Devotion";
                        player.int = player.devotion;
                        player.devotion = tmp;
                    }
                    _ => panic!("swap fail")
                } 
            }
            2 => {
                let tmp = player.devotion;
                swap1 = "Devotion";
                match player.gen.gen_range(0,2){
                    0 => {
                        swap2 = "Strength";
                        player.devotion = player.strength;
                        player.strength = tmp;
                    }
                    1 => {
                        swap2 = "Intellect";
                        player.devotion = player.int;
                        player.int = tmp;
                    }
                    _ => panic!("swap fail")
                } 
            }
            _ => panic!("out of range for final boss!")
        }
        player.print_purple(&format!("Dark energies flow through the air, distorting reality. Your {} and {} are swapped\n", swap1, swap2));
    }
    if turn_counter % 3 == 0 {
        player.health -= turn_counter/2;
        player.print_purple("The Dark Ent moans out a truely aweful sound. He thrusts out his hands wildly sending a black orb right at you.\n");
        println!("You take {} damage", turn_counter/2);
    }
}




