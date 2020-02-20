use std::io::stdin;
use rand::prelude::*;
pub mod items;
pub mod players;
pub mod enemys;
use enemys::Enemy;
use players::Player;


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
            Some("cast") => println!("spell {}", commands.next().unwrap_or("")),
            Some("stats") => player.display_stats(),
            Some("inv") => player.display_inventory(),
            Some("help") => println!("available commands: attack, cast <spell/buff name>, stats, inv"),

            None => println!("unknown command type help for availible commands"),
            _ => println!("unknown command type help for availible commands"),
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
    player.level_up(3, 0, true);

    println!("
    With a snap dust fills the air and your feet hit the floor. 
    Your eyes are open for the first time. 
    A doorway is doorless across the room from you. 
    You are compelled to go through... \n");

    loop {
        let t1_enemys = enemys::load_t1();
        let mut enemy = t1_enemys[player.gen.gen_range(0, t1_enemys.len())];
        if !combat(&mut player, &mut enemy) {
            println!("You have died");
            break;
        } 
    }
}

