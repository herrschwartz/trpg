use std::io::stdin;
use rand::prelude::*;

struct Enemy {
    name: &'static str,
    health: i32,
    dmg_phys: i32,
    dmg_magic: i32,
    armor: i32,
    speed: i32,
    crit: i8,
    tier: i8,
}

struct Weapon {
    name: &'static str,
    damage: i32,
    speed: i32,
    crit: i8,
}

struct Armor {
    name: &'static str,
    value: i32,
    magic_res: i32,
}

struct Player {
    health: i32,
    max_health: i32,
    mana: i32,
    exp: i32,
    level: i32,
    weapon: Weapon,
    armor: Armor,

    strength: i32,
    int: i32,
    resolve: i32,
    devotion: i32,

    gen: rand::rngs::ThreadRng
}

impl Player {
    pub fn new() -> Player {
        let mut rng = thread_rng();
        Player {
            health: 10,
            max_health: 10,
            mana: 10,
            exp: 0,
            level: 1,
            weapon: Weapon {name: "Fists", damage: 0, speed: 1, crit: 2},
            armor: Armor {name: "Cloth Tunic", value: 0, magic_res: 0},
            strength: rng.gen_range(1,4),
            int: rng.gen_range(1,4),
            resolve: rng.gen_range(1,3),
            devotion: rng.gen_range(1,3),
            gen: rng
        }
    }
    pub fn attack(&mut self, target: &mut Enemy) {
        let mut num_atks: i32 = target.speed / self.weapon.speed;
        if num_atks < 1 { num_atks = 1 };
        let mut num_atks_enemy: i32 = self.weapon.speed / target.speed;
        if num_atks_enemy < 1 {num_atks_enemy = 1}

        for _ in 0..num_atks {
            let dmg_amt = self.weapon.damage + self.gen.gen_range(1, self.strength);
            if self.weapon.crit >= self.gen.gen_range(1, 101) {
                target.health -= dmg_amt * 2;
                println!("you crit the {} with your {} for {} damage! nice", target.name, self.weapon.name, dmg_amt);
            } else {
                target.health -= dmg_amt;
                println!("you hit the {} with your {} for {} damage", target.name, self.weapon.name, dmg_amt);
            }
        }
        if target.health <= 0 {
            return 
        }

        for _ in 0.. num_atks_enemy {
            let dmg_amt = target.dmg_phys + target.dmg_magic + self.gen.gen_range(0, target.tier as i32 + 1);
            if target.crit >= self.gen.gen_range(1, 101) {
                self.health -= dmg_amt * 2;
                println!("The {} crits you for {} damage! ow", target.name, dmg_amt)
            } else {
                self.health -= dmg_amt;
                println!("The {} hits you for {} damage", target.name, dmg_amt)
            }
        }
    }
}

static LEVELS: [i32; 12] = [5, 10, 20, 40, 80, 150, 200, 250, 400, 600, 800, 1000];

fn combat(player: &mut Player) -> bool{
    println!("An smelly rat stands in your way?");
    let mut enemy = Enemy {
        name: "rat",
        health: 8,
        dmg_phys: 1,
        dmg_magic: 0,
        armor: 0,
        speed: 2,
        crit: 3,
        tier: 1
    };
    while enemy.health > 0 {
        if player.health <= 0 {
            return false;
        }

        println!("You have {} Health, what will you do?", player.health);
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => {
                let mut commands = input.trim().split_whitespace();
                match commands.next() {
                    Some("attack") => {player.attack(&mut enemy)},
                    Some("cast") => {println!("spell {}", commands.next().unwrap())},

                    None => {println!("unknown command")},
                    _ => {println!("unknown command")},
                }
            }
            Err(e) => println!("invalid command {}", e),
        }
    }
    true
}

fn main() {
    let mut player = Player::new();
    println!("With a snap your feet hit the floor and your eyes open for the first time. A door is sligthly open in front of you. You are compelled to go through");

    if !combat(&mut player) {
        println!("You have died")
    } else {
        println!("you have won")
    }

}

