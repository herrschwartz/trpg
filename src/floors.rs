use crate::enemys::Enemy;
use crate::items::Spell;
use rand::prelude::*;

pub struct Floor {
    pub floor_number: i32,
    pub enemys: Vec<Enemy>,
    pub boss: Enemy,
    pub spells: Vec<Spell>,
    pub rooms: Vec<i32>,
}

impl Floor {
    pub fn new(floor_number: i32) -> Floor {
        let mut rng = thread_rng();
        if floor_number == 1 {
            let enemys = Enemy::load_t1();
            let spells = Spell::load_t1_spells();
            let mut rooms: Vec<i32> = vec![1, 1, 1, 1, 2];
            rooms.shuffle(&mut rng);
            return Floor {
                floor_number,
                enemys,
                spells,
                boss: Enemy{
                    name: "Foreman",
                    health: 20,
                    dmg_phys: 2,
                    dmg_magic: 0,
                    armor: 1,
                    magic_res: 1,
                    speed: 2,
                    crit: 3,
                    tier: 2,
                    atk_txt: "hammers",
                    entry_txt: "
                The labrinth of rooms ends here and opens up into a great hall.
                A battering ram has penetrated the door that seals the vault.
                The foreman stands in front of the drill, reading a piece of paper.
                He looks up at you and sneers. 
                \"You really think you can stop this operation?\" \n"
                },
                rooms
            }
        } else if floor_number == 2 {
            let enemys = Enemy::load_t2();
            let spells = Spell::load_t1_spells();
            let mut rooms: Vec<i32> = vec![1, 1, 1, 1, 1, 2];
            rooms.shuffle(&mut rng);
            return Floor {
                floor_number,
                enemys,
                spells,
                boss: Enemy {
                    name: "Foreman",
                    health: 20,
                    dmg_phys: 2,
                    dmg_magic: 0,
                    armor: 1,
                    magic_res: 1,
                    speed: 2,
                    crit: 3,
                    tier: 1,
                    atk_txt: "hammers",
                    entry_txt: "
                The labrinth of rooms ends here and opens up into a great hall.
                A battering ram has penetrated the door that seals the vault.
                The foreman stands in front of the drill, reading a piece of paper.
                He looks up at you and sneers. 
                \"You really think you can stop this operation?\" \n"
                },
                rooms
            }
        }
        Floor {
            floor_number: 0,
            enemys: vec![],
            spells: vec![],
            boss: Enemy {
                name: "Error",
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
            rooms: vec![],
        }
    }
    pub fn print_entry_txt(&self) {
        match self.floor_number {
            1 => 
    println!("
    With a snap dust fills the air and your feet hit the floor. 
    Your eyes are open for the first time. 
    Roots line the path in front of you.
    The two heavy wooden doors in front of you swing wide open. 
    You are compelled to go through... \n"),
            2 =>
    println!("
    You crawl through the breach into the vault,
    The air is dry inside.
    Great walls of stone joined at perpendicular angles surround you.
    There is an obvious path set out before where the intruders have smashed throught the vault doors...
    \n"),
    _ => panic!("Your in limbo, something fucked up")
        }
    }
}