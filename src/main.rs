use std::io::stdin;
use rand::prelude::*;
pub mod items;
pub mod players;
pub mod enemys;
pub mod floors;
use floors::Floor;
use enemys::Enemy;
use players::Player;
// use std::{thread, time};

static LEVELS: [i32; 12] = [5, 10, 20, 40, 80, 150, 200, 250, 400, 600, 800, 1000];

// fn get_input() -> Vec<String> {
//     let mut input = String::new();
//     stdin().read_line(&mut input).expect("Input Error");

//     let cleaned = input.trim().to_lowercase();
//     let mut commands: Vec<String> = cleaned.split_whitespace().map(|x| x.to_string()).collect();
//     commands
// }

fn combat(player: &mut Player, mut enemy: &mut Enemy) -> bool {
    println!("{}", enemy.entry_txt);
    println!(" {:-<15}+", "+");
    println!(" {:^15}", "Combat");
    println!(" {:-<15}+\n", "+");
    while enemy.health > 0 {
        if player.health <= 0 {
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
            Some("stats") => player.display_stats(),
            Some("inv") => player.display_inventory(),
            Some("help") => println!("available commands: attack, cast <spell/buff name>, stats, inv"),

            None => println!("unknown command type 'help' for availible commands"),
            _ => println!("unknown command, type 'help' for availible commands"),
        }
    }
    println!("The {} is defeated!", enemy.name);
    println!(" {:-<15}+", "+");
    println!(" {:^15}", "Combat Ends");
    println!(" {:-<15}+\n", "+");

    //combat victory end phase
    //restore health
    player.health += player.resolve;
    if player.health > player.max_health {
        player.health = player.max_health
    }

    //grant experiance and level if needed
    player.exp += enemy.tier * 2;
    if player.exp >= LEVELS[(player.level - 1) as usize] {
        println!("Ding! you leveled up!");
        player.level_up(1, 1, false);
    }
    true
}

fn main() {
    let mut player = Player::new();
    println!("Welcome, at any point you can enter 'help' to get information on commands");
    player.level_up(3, 0, true);

    let mut floor = Floor::new(1);
    floor.print_entry_txt();
    loop {
        match floor.rooms.pop() {
           Some(1) => {
                let mut enemy = floor.enemys[player.gen.gen_range(0, floor.enemys.len())];
                if !combat(&mut player, &mut enemy) {
                    println!("You have died");
                    break;
                } 
            },
            Some(2) => {
                let item = floor.spells[player.gen.gen_range(0, floor.spells.len())].clone();
                println!("It's an item room, you got {}", item.name);
                player.spells.push(item);
            },
            None => {
                println!("its time to fight a boss and switch floors");
                combat(&mut player, &mut floor.boss);
                break;
            },
            _ => panic!("Something has gone horribly wrong")
        }
    }
}

