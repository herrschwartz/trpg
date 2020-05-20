use std::io::stdin;
use rand::prelude::*;
pub mod items;
pub mod players;
pub mod enemys;
pub mod floors;
use floors::Floor;
use enemys::Enemy;
use players::Player;
use std::{thread, time};
use items::remove_effect;

static LEVELS: [i32; 12] = [5, 10, 20, 35, 50, 65, 85, 100, 200, 600, 800, 1000];

fn combat(player: &mut Player, mut enemy: &mut Enemy) -> bool {
    println!("{}", enemy.entry_txt);
    println!(" {:-<15}+", "+");
    println!(" {:^15}", "Combat");
    println!(" {:-<15}+\n", "+");
    let mut active_effects: Vec<&'static str> = Vec::new();
    let mut turn_counter = 1;
    while enemy.health > 0 {
        if player.health <= 0 {
            println!("You have died");
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Input Error");
            return false;
        }

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Input Error");

        let cleaned = input.trim().to_lowercase();
        let mut commands = cleaned.split_whitespace();
        match commands.next() {
            Some("attack") => player.attack(&mut enemy),
            Some("cast") => player.cast_spell(&mut enemy, commands.map(|x| x.to_string())
                                                                            .collect::<Vec<String>>()
                                                                            .join(" ")),
            Some("invoke") => {
            match player.invoke_combat(&mut enemy, commands.map(|x| x.to_string()).collect::<Vec<String>>().join(" ")) {
                    Some(b) => active_effects.push(b),
                    None => (),
                }
            },                                                                
            Some("stats") => player.display_stats(),
            Some("inv") => player.display_inventory(),
            Some("help") => println!("available commands: attack, cast 'spell name', invoke 'blessing name', stats, inv (displays inventory)"),

            None => println!("unknown command type 'help' for availible commands"),
            _ => println!("unknown command, type 'help' for availible commands"),
        }

        // BOSS stuff, jank
        if enemy.name == "Dark Figure" {
            if turn_counter == 1 {
                player.print_purple("The dark figure holds out its hand and conjures a weapon that mirrors yours.\n");
                enemy.crit = player.weapon.crit;
                enemy.atk_txt = player.weapon.atk_txt;
                enemy.speed = player.weapon.speed;
            }
        }
        if enemy.name == "Dark Ent" {
            enemys::dark_ent(turn_counter, player);
        }
        if enemy.name == "Sanctum Guardian" {
            enemys::sanctum_guardian(turn_counter, enemy, player);
        }
        turn_counter += 1;
    }
    thread::sleep(time::Duration::from_millis(600));
    println!("The {} is defeated!", enemy.name);

    // ---- combat victory end phase ----
    //restore health
    player.heal(player.resolve);
    thread::sleep(time::Duration::from_millis(600));

    //remove active effects
    for e in active_effects {
        remove_effect(player, e);
        println!("{}",e);
    }

    let lf = enemy.tier * 10 - player.gen.gen_range(0, (enemy.tier * 10) / 2 + 1);
    println!("You absorb {} lifeforce from your enemy", lf);
    player.lifeforce += lf;
    thread::sleep(time::Duration::from_millis(600));
    //grant experiance and level if needed
    player.give_exp(enemy.tier * 2);

    true
}

fn main() {
    let mut player = Player::new();
    print!("Welcome, at any point you can enter ");
    player.print_yellow("'help' ");
    println!("to get information on commands");
    player.level_up(3, 0, true);

    let mut floor = Floor::new(1);
    floor.print_entry_txt();
    loop {
        match floor.rooms.pop() {
           //Combat
           Some(1) => {
                let mut enemy = floor.enemys[player.gen.gen_range(0, floor.enemys.len())];
                if !combat(&mut player, &mut enemy) {
                    break;
                } 
            },
            //Item Room
            Some(2) => {
                floor.item_room(&mut player);
            },
            //Shop
            Some(3) => {
                floor.shop(&mut player);
            }
            Some(4) => {
                floor.special_room(&mut player);
            }
            //Once all rooms are completed, boss time
            None => {
                if !combat(&mut player, &mut floor.boss) {
                    break;
                }
                if floor.floor_number == 3 {
                    let mut final_boss_p2 = Enemy::final_boss_phase_2();
                    thread::sleep(time::Duration::from_millis(3500));
                    if !combat(&mut player, &mut final_boss_p2) {
                        break;
                    }
                    println!("It's finally over. The dark intruders have all been defeated. 
                    You collapse to your knees. Your essence is returns to the great mother as you wilt to dust.");
                    let mut input = String::new();
                    stdin().read_line(&mut input).expect("Input Error");
                    break;
                }
                floor = Floor::new(floor.floor_number + 1);
                floor.print_entry_txt();
            },
            _ => panic!("Something has gone horribly wrong")
        }
        let spell_chance = ((player.devotion as f64).log2() * 10.0) as i32 + player.devotion;
        if spell_chance >= player.gen.gen_range(1, 101) {
            println!("\nYour devotion has paid off,\n    ...your favor grants you knowledge");
            match player.gen.gen_range(0,2) {
                0 => floor.get_random_blessing(&mut player, 1),
                1 => floor.get_random_spell(&mut player, 1),
                _ => panic!("Out of bounds for devotion chance")
            }
        }
        rest(&mut player, &floor);
    }
}

fn rest(player: &mut Player, floor: &Floor) {
    println!("You have a moment to gather yourself, if you need it.");
    if floor.rooms.len() == 0 {
        print!("The room ahead looks ");
        player.print_red("dangerous\n");
    }
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Input Error");

        let cleaned = input.trim().to_lowercase();
        let mut commands = cleaned.split_whitespace();
        match commands.next() {
            Some("next") => break,
            Some("equip") => player.equip(commands.map(|x| x.to_string()).collect::<Vec<String>>().join(" ")),
            Some("invoke") => player.invoke_non_combat(commands.map(|x| x.to_string()).collect::<Vec<String>>().join(" ")),     
            Some("stats") => player.display_stats(),
            Some("inv") => player.display_inventory(),
            Some("help") => println!("available commands: next (advances to the next room), invoke 'blessing name', equip 'weapon name', stats, inv (displays inventory)"),

            None => println!("unknown command type 'help' for availible commands"),
            _ => println!("unknown command, type 'help' for availible commands"),
        }
    }
}


