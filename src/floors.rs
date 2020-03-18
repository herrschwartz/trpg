use crate::enemys::Enemy;
use crate::items::{Spell,Blessing,Weapon};
use crate::players::Player;
use rand::prelude::*;
use std::io::stdin;
use std::{thread, time};

pub struct Floor {
    pub floor_number: i32,
    pub enemys: Vec<Enemy>,
    pub boss: Enemy,
    pub spells: Vec<Spell>,
    pub blessings: Vec<Blessing>,
    pub weapons: Vec<Weapon>,
    pub rooms: Vec<i32>,
}

static CHESTS: [&'static str; 5] = ["Gold Chest", "Blue Chest", "Grey Chest", "Purple Chest", "Green Chest"];

impl Floor {
    pub fn new(floor_number: i32) -> Floor {
        let mut rng = thread_rng();
        if floor_number == 1 {
            let enemys = Enemy::load_t1();
            let spells = Spell::load_t1_spells();
            let blessings = Blessing::load_t1_blessings();
            let weapons = Weapon::load_t1_weapons();
            let mut rooms: Vec<i32> = vec![1, 1, 1, 1, 2];
            rooms.shuffle(&mut rng);
            rooms.insert(rng.gen_range(0,2), 3);
            return Floor {
                floor_number,
                enemys,
                spells,
                blessings,
                weapons,
                boss: Enemy{
                    name: "Foreman",
                    health: 25,
                    dmg_phys: 1,
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
            let spells = Spell::load_t2_spells();
            let blessings = Blessing::load_t1_blessings(); //TODO: change when designing floor 2
            let weapons = Weapon::load_t1_weapons(); //TODO: See above
            let mut rooms: Vec<i32> = vec![1, 1, 1, 1, 1, 2, 3];
            rooms.shuffle(&mut rng);
            return Floor {
                floor_number,
                enemys,
                spells,
                blessings,
                weapons,
                boss: Enemy {
                    name: "Sanctum Guardian",
                    health: 40,
                    dmg_phys: 2,
                    dmg_magic: 0,
                    armor: 5,
                    magic_res: 0,
                    speed: 2,
                    crit: 3,
                    tier: 3,
                    atk_txt: "slams",
                    entry_txt: "
                       FILL IN TEXT  \n"
                },
                rooms
            }
        }
        Floor {
            floor_number: 0,
            enemys: vec![],
            spells: vec![],
            weapons: vec![],
            blessings: vec![],
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
                entry_txt: ""
            },
            rooms: vec![],
        }
    }

    pub fn get_random_spell(&self, player: &mut Player, amount: i32) {
        for _ in 0..amount {
            let spell = self.spells[player.gen.gen_range(0, self.spells.len())].clone();
            println!("You recieved {}", spell.name);
            player.spells.push(spell);
            thread::sleep(time::Duration::from_millis(600));
        }
    }

    pub fn get_random_blessing(&self, player: &mut Player, amount: i32) {
        for _ in 0..amount {
            let blessing = self.blessings[player.gen.gen_range(0, self.blessings.len())].clone();
            println!("You recieved {}", blessing.name);
            player.blessings.push(blessing);
            thread::sleep(time::Duration::from_millis(600));
        }
    }

    pub fn get_random_weapon(&self, player: &mut Player, amount: i32) {
        for _ in 0..amount {
            let weapon = self.weapons[player.gen.gen_range(0, self.weapons.len())].clone();
            println!("You recieved {}", weapon.name);
            player.weapons.push(weapon);
            thread::sleep(time::Duration::from_millis(600));
        }
    }

    pub fn item_room(&self, player: &mut Player) {
        let mut chests = CHESTS.to_vec();
        chests.shuffle(&mut player.gen);
        let chest1 = chests.pop().unwrap();
        let chest2 = chests.pop().unwrap();
        println!("
        You enter a circular room with a raised platform in the middle. 
        there is a {} and a {} on the alter, 
        which would you like to open?", chest1, chest2);
        loop {
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Input Error");
            let cleaned = input.trim().to_lowercase();
            let choice = cleaned.as_str();
            if choice == chest1.to_lowercase() || choice == chest2.to_lowercase() {
                match choice {
                    "gold chest" => self.get_random_blessing(player, 1),
                    "blue chest" => self.get_random_spell(player, 2),
                    "grey chest" => self.get_random_weapon(player, 1),
                    "purple chest" => {
                        for _ in 0..2 {
                            match player.gen.gen_range(0,3) {
                                0 => self.get_random_blessing(player, 1),
                                1 => self.get_random_spell(player, 2),
                                2 => self.get_random_weapon(player, 1),
                                _ => panic!("Out of Range Item in random chest"),
                            }
                        }
                    }
                    "green chest" => {
                        match player.gen.gen_range(0,2) {
                            0 => self.get_random_blessing(player, 2),
                            1 => self.get_random_spell(player, 3),
                            _ => panic!("Out of range in spell chest"),
                        }
                    }
                    _ => panic!("Chest does not exist {}", choice),
                }
                break;
            } else {
                println!("There is no chest there named {}", input);
            }
        }
    }

    pub fn shop(&self, player: &mut Player) {
        println!("
        Along the side of the hallways you notice a glowing alcove. 
        This must be an ancient forge used by your ancestors.
        The forge can strengthen your body or conjure magic and item from lifeforce.\n");

        let mut for_sale = vec![("Armor Upgrage", "upgrade", 16 * self.floor_number), ("Magic Armor Upgrade", "upgrade", 11 * self.floor_number)];

        for _ in 0..player.gen.gen_range(4, 6) {  
            match player.gen.gen_range(0,5) {
                0 | 1 => for_sale.push((self.blessings[player.gen.gen_range(0, self.blessings.len())].name, "blessing", player.gen.gen_range(6,10) * self.floor_number)),
                2 | 3 => for_sale.push((self.spells[player.gen.gen_range(0, self.spells.len())].name, "spell", player.gen.gen_range(6,10) * self.floor_number)),
                4     => for_sale.push((self.weapons[player.gen.gen_range(0, self.weapons.len())].name, "weapon", player.gen.gen_range(12, 14) * self.floor_number)),
                _     => panic!("Out of range random value")
            }
       }

       println!("Lifeforce: {}", player.lifeforce);
       for (i, item) in for_sale.iter().enumerate() {
           println!("{}) {:<20} {:<3}", i+1, item.0, item.2)
       }

       loop {
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Input Error");

            let cleaned = input.trim().to_lowercase();
            let mut commands = cleaned.split_whitespace();
            let cmd = commands.next().unwrap_or_default();

            match cmd {
                "1" | "2" | "3" | "4" | "5" | "6" | "7" => {
                    let n: usize = cmd.parse().unwrap();
                    if n <= for_sale.len() {
                        let buying = for_sale[n-1];
                        if player.lifeforce < buying.2 {
                            println!("Not enough lifeforce for {}", buying.0);
                            continue;
                        }
                        match buying.1 {
                            "upgrade" => {
                                if buying.0 == "Armor Upgrage" {
                                    player.armor += 1;
                                    println!("Your armor increases by 1");
                                } else {
                                    player.armor_magic += 1;
                                    println!("Your magic armor increases by 1");
                                }
                            }
                            "blessing" => {
                                player.blessings.push(self.blessings.iter().find(|x| x.name == buying.0).unwrap().clone());
                                println!("You recieve {}", buying.0);
                            }
                            "spell" => {
                                player.spells.push(self.spells.iter().find(|x| x.name == buying.0).unwrap().clone());
                                println!("You recieve {}", buying.0);
                            }
                            "weapon" => {
                                player.weapons.push(self.weapons.iter().find(|x| x.name == buying.0).unwrap().clone());
                                println!("You recieve {}", buying.0);
                            }
                            _=> panic!("Invalid Shop item type")
                        }
                        player.lifeforce -= buying.2;
                        for_sale.remove(n-1);
                        println!("Lifeforce: {}", player.lifeforce);
                        for (i, item) in for_sale.iter().enumerate() {
                            println!("{}) {:<20} {:<3}", i+1, item.0, item.2)
                        }
                    }
                }
                "inv"    => player.display_inventory(),
                "stats"  => player.display_stats(),
                "leave"  => break,
                "relist" => {
                    for (i, item) in for_sale.iter().enumerate() {
                        println!("{}) {:<20} {:<3}", i+1, item.0, item.2)
                    }
                },
                "help"   => println!("Enter the number of the item that you wish to buy, type 'leave' to leave the shop.\n inv, stats, relist (relists shop items)"),
                _ => println!("Invalid command {}, Enter the number of the item that you wish to buy, type 'leave' to leave the shop", cmd),
            }
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
    _ => panic!("You're in limbo, something fucked up")
        }
    }
}