use crate::enemys::Enemy;
use crate::items::{Spell,Blessing,Weapon};
use crate::players::Player;
use crate::combat;
use rand::prelude::*;
use std::io::stdin;
use std::{thread, time};
use crate::end_game;

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

pub fn get_single_command() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Input Error");

    let cleaned = input.trim().to_lowercase();
    let mut commands = cleaned.split_whitespace();
    let cmd = commands.next().unwrap_or_default();
    cmd.to_owned()
}

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
            let blessings = Blessing::load_t2_blessings();
            let weapons = Weapon::load_t2_weapons(); 
            let mut rooms: Vec<i32> = vec![1, 1, 1, 1, 1, 2, 4, 4];
            let mut special_rooms: Vec<&'static str> = vec!["enchant", "sacrafice2", "hilt", "resolve"];
            rooms.shuffle(&mut rng);
            special_rooms.shuffle(&mut rng);
            rooms.insert(rng.gen_range(0,3), 3);
            return Floor {
                floor_number,
                enemys,
                spells,
                blessings,
                weapons,
                special_rooms,
                boss: Enemy {
                    name: "Sanctum Guardian",
                    health: 46,
                    dmg_phys: 3,
                    dmg_magic: 0,
                    armor: 10,
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
                       The guardian activates, its armor looks thick
                       ERADICATE. INTRUDERS. \n"
                },
                rooms
            }
        } else if floor_number == 3 {
            let enemys = Enemy::load_t3();
            let spells = Spell::load_t3_spells();
            let blessings = Blessing::load_t3_blessings(); 
            let weapons = Weapon::load_t3_weapons(); 
            let mut rooms: Vec<i32> = vec![1, 1, 1, 1, 1, 2, 4, 4];
            let mut special_rooms: Vec<&'static str> = vec!["sacrafice3", "blade", "magician", "trivia"];
            rooms.shuffle(&mut rng);
            rooms.insert(rng.gen_range(0,3), 3);
            special_rooms.shuffle(&mut rng);
            return Floor {
                floor_number,
                enemys,
                spells,
                blessings,
                weapons,
                special_rooms,
                boss: Enemy {
                    name: "Dark Figure",
                    health: 46,
                    dmg_phys: 3,
                    dmg_magic: 0,
                    armor: 3,
                    magic_res: 2,
                    speed: 2,
                    crit: 5,
                    tier: 3,
                    atk_txt: "punches",
                    entry_txt: "
                       This is it.
                       You have mad it to the heart of the great mother.
                       Her roots drape down all around you and the gold-blue glow of the heart draws you closer.
                       The dark figure from before steps in between you, it is the reason why you are here. \n"
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
            player.score += 50;
            thread::sleep(time::Duration::from_millis(600));
        }
    }

    pub fn get_random_blessing(&self, player: &mut Player, amount: i32) {
        for _ in 0..amount {
            let blessing = self.blessings[player.gen.gen_range(0, self.blessings.len())].clone();
            println!("You recieved {}", blessing.name);
            player.blessings.push(blessing);
            player.score += 50;
            thread::sleep(time::Duration::from_millis(600));
        }
    }

    pub fn get_random_weapon(&self, player: &mut Player, amount: i32) {
        for _ in 0..amount {
            let weapon = self.weapons[player.gen.gen_range(0, self.weapons.len())].clone();
            println!("You recieved {}", weapon.name);
            player.weapons.push(weapon);
            player.score += 100;
            thread::sleep(time::Duration::from_millis(600));
        }
    }

    pub fn get_choice(&self, choices: Vec<&str>, player: &Player,) -> usize {
        for (i, item) in choices.iter().enumerate() {
            println!("{}) {}", i+1, item)
        }
        
        loop {
            let cmd = get_single_command();
            let choice = cmd.as_str();
            match choice {
                "1" | "2" | "3" | "4" | "5" | "6" | "7" => {
                    let n: usize = cmd.parse().unwrap();
                    if n <= choices.len() {
                        return n;
                    }
                }
                "inv"    => player.display_inventory(),
                "stats"  => player.display_stats(),
                "relist" => {
                    for (i, item) in choices.iter().enumerate() {
                        println!("{}) {}", i+1, item)
                    }
                }
                "help"   => println!("Enter the number of the choice you wish to choose, \ncommands: inv, stats, relist (relists choices)"),
                _ => println!("Invalid command {} - Enter the number of the choice you wish to choose, type help for availible commands", cmd),
            }
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
                if player.gen.gen_range(0,20) == 19 {
                    if !combat(player, &mut Enemy {
                        name: "Mimic",
                        health: 4 + self.floor_number * 6,
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
                        end_game(player);
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
                        player.score += 100;
                        player.lifeforce += 10;
                        println!("You gain 10 lifeforce");
                    }
                    _ => panic!("Chest does not exist {}", choice),
                }
                player.score += 50;
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
            let cm = get_single_command();
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

                match self.get_choice(vec!["Ask her to heal you (+6 health)", 
                                           "Ask her for her enchantment (+3 max health)",
                                           "Slay the hag (+5 lifeforce)"], player) 
                {
                    1 => player.heal(6),
                    2 => {
                        player.max_health += 3;
                        println!("The old lady mutters some ancient words you don't know");
                        println!("Your maximum health increases by 3");
                    },
                    3 => player.give_lifeforce(5),
                    _ => panic!("Error hag out of range"),
                }
            },
            Some("specialize") => {
                println!("
                You push your way through thick roots to get past though this room. 
                Almost through you see the roots move in a perculiar way. 
                They twist into the shape of a face before your eyes. 
                \"Ahhh young one, you have a great task ahead of you. What will you make of it?\"\n");

                match self.get_choice(vec!["I want to be a brave warrior", 
                                           "I will be a devout follower of the great mother",
                                           "I wish to be a powerful sorcerer"], player) 
                {
                    1 => {
                        println!("Good ... I will bring out your inner stength so that you may fight then");
                        thread::sleep(time::Duration::from_millis(600));
                        player.resolve += 1;
                        println!("Your resolve increases by 1");
                    }
                    2 => {
                        println!("Ahhh ... I am glad to hear it young one, blessed be your path");
                        thread::sleep(time::Duration::from_millis(600));
                        self.get_random_blessing(player, 1);
                        player.devotion += 1;
                        println!("Your devotion increases by 1");
                    }
                    3 => {
                        println!("hmmm ... unconventional for our kind, but so be it");
                        thread::sleep(time::Duration::from_millis(600));
                        self.get_random_spell(player, 2);
                        player.int += 1;
                        println!("Your intellect increases by 1");
                    }
                    _ => panic!("Out of bound for specialize"),
                }
            },
            Some("sacrafice") => {
                println!("
                Along one of the hall you spot a glowing red orb
                You may be able to absorb some lifeforce from it, but it will certainly cost you.
                ");
                match self.get_choice(vec![
                    "Touch the orb (-3 health, +4 Lifeforce)",
                    "Palm the orb (-5 health, +7 Lifeforce)",
                    "It looks evil (leave)",
                ], player) {
                    1 => { 
                        println!("You take 3 damage");
                        player.give_lifeforce(4);
                        player.health -= 3;
                    }
                    2 => { 
                        println!("You take 5 damage");
                        player.health -= 5;
                        player.give_lifeforce(7);
                    }
                    3 => println!("You ignore the orb an hurry along the path"),
                    _ => panic!("sacrafice out of bounds"),
                }
                if player.health <= 0 {
                    end_game(player);
                }
            }
            Some("sacrafice2") => {
                println!("
                Along one of the hall you spot a glowing red orb
                You may be able to absorb some lifeforce from it, but it will certainly cost you.
                ");
                match self.get_choice(vec![
                    "Touch the orb (-4 health, +6 Lifeforce)",
                    "Palm the orb (-6 health, +8 Lifeforce)",
                    "It looks evil (leave)",
                ], player) {
                    1 => { 
                        println!("You take 4 damage");
                        player.give_lifeforce(6);
                        player.health -= 4;
                    }
                    2 => { 
                        println!("You take 6 damage");
                        player.health -= 6;
                        player.give_lifeforce(8);
                    }
                    3 => println!("You ignore the orb an hurry along the path"),
                    _ => panic!("sacrafice out of bounds"),
                }
                if player.health <= 0 {
                    end_game(player);
                }
            }
            Some("enchant") => {
                println!("
                You notice this room is brighter than the others,
                A shaft of brilliant blue light beams through the center.
                Maybe putting your weapon in it will have and effect?
                ");
                match self.get_choice(vec![
                    "Enchant you weapon (10 lifeforce)",
                    "leave",
                ], player) {
                    1 => {
                        if player.lifeforce >= 10 {
                            println!("You dip your {} in the light, its critical strike chance increases by 5%", player.weapon.name);
                            player.weapon.crit += 5;
                            player.lifeforce -= 10;
                        }
                        else {
                            println!("You don't have enough lifeforce to enchant your weapon");
                        }
                    }
                    2 => println!("Theres no time to mess with that."),
                    _ => panic!("sacrafice out of bounds"),
                }
            }
            Some("resolve") => {
                println!("
                A figure comes out of nowhere and grabs you by your shoulders,
                \"Do you have what it takes, can you save us?\"
                His knareled hands dig into you.
                ");
                match self.get_choice(vec![
                    "I do, I will win (requires 4 resolve)",
                    "I'm not sure",
                    "leave",
                ], player) {
                    1 => {
                        if player.resolve >= 4 {
                            println!("\"Good I believe you can do it!\"");
                            player.max_health += 4;
                            player.print_yellow("Your maximum health increases by 4");
                            player.heal(5);
                        }
                        else {
                            println!("\"I don't believe you young one\"");
                        }
                    }
                    2 => {
                        player.heal(4);
                        println!("\"You should have more faith in yourself\"")
                    }
                    3 => println!("You free yourself from his grasp and hurry away."),
                    _ => panic!("sacrafice out of bounds"),
                }
            }
            Some("hilt") => {
                println!("
                A dimly illuminated alter glows in the darkness,
                There is an ornate blade hilt on the alter next to a pile of scrolls.
                ");
                match self.get_choice(vec![
                    "Take the hilt",
                    "take the scrolls",
                ], player) {
                    1 => {  
                        println!("You take the old hilt, but it doesn't seem very useful for now");
                        player.hilt = true;
                    }
                    2 => {
                        println!("you grab the dusty old scolls");
                        println!("you receive Ancient Knowledge");
                        self.get_random_spell(player, 1);
                        player.blessings.push(
                            Blessing {
                                name: "Ancient Knowledge",
                                description: "increases your intellect permanently",
                                speed: 1,
                                retaliation: false,
                                combat_only: false,
                                active_effect: false,
                                invoke_txt: "The information contained within this scroll is truely profound"
                            }
                        )
                    }
                    _ => panic!("sacrafice out of bounds"),
                }
            }
            Some("sacrafice3") => {
                println!("
                Along one of the hall you spot a glowing red orb
                You may be able to make a trade with blood.
                ");
                match self.get_choice(vec![
                    "Touch the orb (-5 health, +3 Max Health)",
                    "Palm the orb (-8 health, +5 Max Health)",
                    "It looks evil (leave)",
                ], player) {
                    1 => { 
                        println!("You take 5 damage");
                        player.max_health += 3;
                        player.health -= 5;
                    }
                    2 => { 
                        println!("You take 8 damage");
                        player.health -= 8;
                        player.max_health += 5;
                    }
                    3 => println!("You ignore the orb an hurry along the path"),
                    _ => panic!("sacrafice out of bounds"),
                }
                if player.health <= 0 {
                    end_game(player);
                }
            }
            Some("blade") => {
                println!("
                A dimly illuminated alter glows in the darkness,
                There is an engraved blade with with no hilt on the alter next to a book.
                ");
                match self.get_choice(vec![
                    "Take the blade",
                    "take the book",
                ], player) {
                    1 => {  
                        if player.hilt {
                            println!("You grab the blade, this must go with the hilt you found earlier. \n sure enough the two fit perfectly together");
                            player.weapons.push(Weapon {
                                name: "Onodreem's Great Blade",
                                damage: 5,
                                speed: 2,
                                crit: 10,
                                rank: 1,
                                crit_txt: "Reign down slices upon",
                                atk_txt: "Gracefully Cut",
                            });
                            println!("You recieve Onodreem's Great Blade");
                            player.score += 500;
                        } else {
                            println!("You take the blade being careful not to cut yourself");
                        }
                    }
                    2 => {
                        println!("you grab the dusty old book");
                        println!("you receive The Entobile");
                        self.get_random_spell(player, 1);
                        player.blessings.push(
                            Blessing {
                                name: "The Entobile",
                                description: "The Holy book of the Ents, few copies exist",
                                speed: 1,
                                retaliation: false,
                                combat_only: false,
                                active_effect: false,
                                invoke_txt: "Reading the passages contained within the tomb both touch and enlighten you"
                            }
                        )
                    }
                    _ => panic!("sacrafice out of bounds"),
                }
            }
            Some ("magician") => {
                println!("
                You come across a sharply dressed woman, leaning against the trunk on the side of the ramp
                How did she get down here?
                \"Hey there, I offed a cleric a ways back but these holy spells are no good for me. All I want in return is a real magic spell of my choice.\" 
                She smirks at you, waiting for your response.");
                match self.get_choice(vec![
                    "Accept her offer (-1 spell, gain blessings)",
                    "Politely decline (leave)",
                    "Slay the witch (+1 spell)",
                ], player) {
                    1 => {
                        let sp = player.spells.remove(player.gen.gen_range(0, player.spells.len()));
                        println!("{} Removed", sp.name);
                        player.blessings.push(            
                            Blessing {
                                name: "Heal",
                                description: "A basic heal that scales with devotion",
                                speed: 1,
                                retaliation: true,
                                combat_only: false,
                                active_effect: false,
                                invoke_txt: "You bask in holy light, restoring your vitality"
                            }
                        );
                        player.blessings.push(           
                            Blessing {
                                name: "Greater Heal",
                                description: "A heal that scales with devotion, slower than Heal",
                                speed: 2,
                                retaliation: true,
                                combat_only: false,
                                active_effect: false,
                                invoke_txt: "You bask in a wave of Holy Light, restoring your vitality"
                            }
                        );
                        thread::sleep(time::Duration::from_millis(600));
                        println!("You recieved Greater Heal");
                        thread::sleep(time::Duration::from_millis(600));
                        println!("You recieved Heal");
                        thread::sleep(time::Duration::from_millis(600));
                        println!("\"A good trade indeed.\" She gives you a wink");
                    }
                    2 => println!("You decline and continue on the path"),
                    3 => {
                        //combat
                        if !combat( player, &mut Enemy {
                                name: "Dark Magician Girl",
                                health: 26,
                                dmg_phys: 0,
                                dmg_magic: 5,
                                armor: 2,
                                magic_res: 4,
                                speed: 2,
                                crit: 6,
                                tier: 3,
                                atk_txt: "casts a firball at",
                                entry_txt: "\"Fine then, looks like I'll just have to take you out too\" \n"
                            }
                        ) {
                            end_game(player);
                        }
                        self.get_random_spell(player, 1);
                    }
                    _ => panic!("Error in Magician event!")
                }
            }         
            Some("trivia") => {
                println!("
                An hunched old man come running up to you with a crazed look.
                One eye looks right at you, the other somewhere off in the distance
                \"Ahh traveler, answer my question correctly and recieve a prize!\""
                );
                match self.get_choice(vec![
                    "Ok", 
                    "Slay the old man",    
                ], player) {
                    1 => println!(),
                    2 => println!("You swing your weapon at the old man but it passes right through him \n\"Heheheh, ok funny guy. On with the questions!\" "),
                    _ => panic!("old man"),
                }
                let mut correct = false;
                match player.gen.gen_range(0, 3){
                    0 => {
                        println!("How many planets are directly visable to the naked eye at night?");
                        match self.get_choice(vec![
                            "4", 
                            "5",  
                            "6",  
                        ], player) {
                            1 => println!("\"Sorry, try again next time! heheheh\""),
                            2 => correct = true,
                            3 => println!("\"Sorry, try again next time! heheheh\""),
                            _ => panic!("planet error")
                        }
                    }
                    1 => {
                        println!("How many years does it take for a Red Oak to start producing acorns?");
                        match self.get_choice(vec![
                            "20-30 years", 
                            "40-50 years",  
                            "90-100 years",  
                        ], player) {
                            1 => correct = true,
                            2 => println!("\"Sorry, try again next time! heheheh\""),
                            3 => println!("\"Sorry, try again next time! heheheh\""),
                            _ => panic!("acorn error")
                        }
                    }
                    2 => {
                        println!("Which of these is not a true berry?");
                        match self.get_choice(vec![
                            "Strawberry", 
                            "Blueberry",  
                            "Banana",  
                        ], player) {
                            1 => correct = true,
                            2 => println!("\"Sorry, try again next time! heheheh\""),
                            3 => println!("\"Sorry, try again next time! heheheh\""),
                            _ => panic!("berry error")
                        }
                    }
                    3 => {
                        println!("The bark of which of these trees is a painkiller?");
                        match self.get_choice(vec![
                            "Aspen", 
                            "Birch",  
                            "Willow",  
                        ], player) {
                            1 => println!("\"Sorry, try again next time! heheheh\""),
                            2 => println!("\"Sorry, try again next time! heheheh\""),
                            3 => correct = true,
                            _ => panic!("berry error")
                        }
                    }
                    _=> panic!("error out of range for old man trivia"),
                }
                if correct {
                    println!("\"Good job, You got it right!\"");
                    self.get_random_blessing(player, 1);
                    player.heal(10);
                    thread::sleep(time::Duration::from_millis(600));
                    println!("The old man fades away before your eyes, was he just an illusion?");
                    thread::sleep(time::Duration::from_millis(600));
                    player.give_lifeforce(15);
                }
            }
            
            None => println!("Somehow all of the special rooms are exhausted"),
            _ => panic!("There was in error in special rooms"),
        }
        player.score += self.floor_number * 50
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