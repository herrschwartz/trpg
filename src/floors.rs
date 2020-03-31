use crate::enemys::Enemy;
use crate::items::{Spell,Blessing,Weapon};
use crate::players::Player;
use crate::combat;
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
    pub special_rooms: Vec<&'static str>,
}

static CHESTS: [&'static str; 6] = ["Gold Chest", "Blue Chest", "Grey Chest", "Purple Chest", "Green Chest", "Black Chest"];

impl Floor {
    pub fn new(floor_number: i32) -> Floor {
        let mut rng = thread_rng();
        if floor_number == 1 {
            let enemys = Enemy::load_t1();
            let spells = Spell::load_t1_spells();
            let blessings = Blessing::load_t1_blessings();
            let weapons = Weapon::load_t1_weapons();
            let mut rooms: Vec<i32> = vec![1, 1, 1, 1, 2, 4];
            let mut special_rooms: Vec<&'static str> = vec!["hag", "sacrafice", "specialize"];
            rooms.shuffle(&mut rng);
            special_rooms.shuffle(&mut rng);
            rooms.insert(rng.gen_range(0,2), 3);
            return Floor {
                floor_number,
                enemys,
                spells,
                blessings,
                weapons,
                special_rooms,
                boss: Enemy{
                    name: "Foreman",
                    health: 25,
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
            let spells = Spell::load_t2_spells();
            let blessings = Blessing::load_t2_blessings();
            let weapons = Weapon::load_t2_weapons(); 
            let mut rooms: Vec<i32> = vec![1, 1, 1, 1, 1, 2];
            let mut special_rooms: Vec<&'static str> = vec!["hag", "sacrafice"]; //TODO: floor 2 rooms
            rooms.insert(rng.gen_range(0,3), 3);
            rooms.shuffle(&mut rng);
            special_rooms.shuffle(&mut rng);
            return Floor {
                floor_number,
                enemys,
                spells,
                blessings,
                weapons,
                special_rooms,
                boss: Enemy {
                    name: "Sanctum Guardian",
                    health: 42,
                    dmg_phys: 3,
                    dmg_magic: 0,
                    armor: 5,
                    magic_res: 0,
                    speed: 3,
                    crit: 3,
                    tier: 3,
                    atk_txt: "slams",
                    entry_txt: "
                       You make your way into the heart of the Sanctuary.
                       walking into the expansive room you see the trunk of the great mother in the middle.
                       The imposing  Guardian stands before it. A figure in all black with a placid mask stands on its shoulder.
                       before you even get a chance to react the figure jumps down a dispears into a hollow in the trunk.
                       The guardian activates
                       ERADICATE. INTRUDERS. \n"
                },
                rooms
            }
        } else if floor_number == 3 {
            let enemys = Enemy::load_t3();
            let spells = Spell::load_t2_spells();
            let blessings = Blessing::load_t2_blessings(); //TODO: change when designing floor 3
            let weapons = Weapon::load_t2_weapons(); //TODO: See above
            let mut rooms: Vec<i32> = vec![1, 1, 1, 1, 1, 1, 2];
            let mut special_rooms: Vec<&'static str> = vec!["hag", "sacrafice"]; //TODO: floor 3 rooms
            rooms.insert(rng.gen_range(0,4), 3);
            rooms.shuffle(&mut rng);
            special_rooms.shuffle(&mut rng);
            return Floor {
                floor_number,
                enemys,
                spells,
                blessings,
                weapons,
                special_rooms,
                boss: Enemy {
                    name: "Sanctum Guardian",
                    health: 42,
                    dmg_phys: 3,
                    dmg_magic: 0,
                    armor: 5,
                    magic_res: 0,
                    speed: 3,
                    crit: 3,
                    tier: 3,
                    atk_txt: "slams",
                    entry_txt: "
                       You make your way into the heart of the Sanctuary.
                       walking into the expansive room you see the trunk of the great mother in the middle.
                       The imposing Guardian stands before it. A figure in all black with a placid mask stands on its shoulder.
                       before you even get a chance to react the figure jumps down a dispears into a hollow in the trunk.
                       The guardian activates, its skin looks hardened.
                       ERADICATE. INTRUDERS. \n"
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
            special_rooms: vec![],
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

    pub fn get_single_command(&self) -> String {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Input Error");

        let cleaned = input.trim().to_lowercase();
        let mut commands = cleaned.split_whitespace();
        let cmd = commands.next().unwrap_or_default();
        cmd.to_owned()
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
                if player.gen.gen_range(0,20) == 19 {
                    if !combat(player, &mut Enemy {
                        name: "Mimic",
                        health: 5 + self.floor_number * 5,
                        dmg_phys: 2,
                        dmg_magic: 0,
                        armor: 1,
                        magic_res: 1,
                        speed: 2,
                        crit: 3,
                        tier: self.floor_number,
                        atk_txt: "bites",
                        entry_txt: "The chest springs to life and leaps at you, trying to devour you with its gaping teeth! \n"
                    }) {
                        panic!(""); //this is lazy af
                    }
                }
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
                    "black chest" => {
                        match player.gen.gen_range(0,3) {
                            0 => {
                                println!("The chest is cursed! you lose 1 resolve");
                                player.resolve -= 1;
                                self.get_random_blessing(player, 2);
                            },
                            1 => {
                                println!("The chest is cursed! you lose 1 intellect");
                                player.int -= 1;
                                self.get_random_weapon(player, 1);
                            },
                            2 => {
                                println!("The chest is cursed! you lose 1 devotion");
                                player.devotion -= 1;
                                self.get_random_spell(player, 3);
                            },
                            _=> panic!("Out of range in cursed chest")
                        }
                        player.lifeforce += 10;
                        println!("You gain 10 lifeforce");
                    }
                    _ => panic!("Chest does not exist {}", choice),
                }
                break;
            } else {
                println!("There is no chest there named {} \n example: green chest", input);
            }
        }
    }

    pub fn shop(&self, player: &mut Player) {
        println!("
        Along the side of the hallways you notice a glowing alcove. 
        This must be an ancient forge used by your ancestors.
        The forge can strengthen your body or conjure magic and item from lifeforce.\n");

        let mut for_sale = vec![("Armor Upgrage", "upgrade", 38 + self.floor_number*2), ("Magic Armor Upgrade", "upgrade", 15 * self.floor_number)];

        for _ in 0..player.gen.gen_range(4, 6) {  
            match player.gen.gen_range(0,5) {
                0 | 1 => for_sale.push((self.blessings[player.gen.gen_range(0, self.blessings.len())].name, "blessing", player.gen.gen_range(7,11) * self.floor_number)),
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
            let cm = self.get_single_command();
            let cmd = cm.as_str();
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

    pub fn special_room(&mut self, player: &mut Player) {
        match self.special_rooms.pop() {
            Some("hag") => {
                println!("
                The room is dim and damp, great roots line the ceiling.
                An old chrone is hunched over in the center of the room.
                \"I can grant you stamina for your journey, if you so desire\"\n");
                println!("1) Ask her to heal you (+6 health)");
                println!("2) Ask her for her enchantment (+3 max health)");
                println!("3) Slay the hag (+5 lifeforce)");
                match self.get_single_command().as_str() {
                    "1" => println!("good"),
                    "2" => println!("nice"),
                    "3" => println!("cool"),
                    "stats" => player.display_stats(),
                    "inv" => player.display_inventory(),
                    _ => println!("not good"),
                }
            },
            Some("specialize") => {
                println!("
                You push your way through thick roots to get past though this room. 
                Almost through you see the roots move in a perculiar way. 
                They twist into the shape of a face before your eyes. 
                \"Ahhh young one, you have a great task ahead of you. What will you make of it?\"\n");
                println!("1) I want to be a brave warrior");
                println!("2) I will be a devout follower of the great mother");
                println!("2) I wish to be a powerful sorcerer");
            },
            Some("sacrafice") => {
                println!("risk/reward");
            }
            None => println!("Somehow all of the special rooms are exhausted"),
            _ => panic!("There was in error in special rooms")
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
            3 => 
    println!("
    You enter the hollow a dark hole leads into the inner sanctum.
    You take the plundge falling faster and faster. 
    Before you hit the ground, your speed starts to slow. 
    Gracefully your feet float down to disrupt the mirrored surface of a shallow pond.
    The flickering of glowing bugs lead the way down the great spiral ramp of the inner sanctum. 
    "),
    _ => panic!("You're in limbo, something fucked up")
        }
    }
}