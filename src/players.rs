use crate::items::Weapon;
use crate::items::Armor;
use crate::enemys::Enemy;
use rand::prelude::*;
use std::io::stdin;

pub struct Player {
    pub health: i32,
    pub max_health: i32,
    pub exp: i32,
    pub level: i32,
    pub weapon: Weapon,
    pub armor: Armor,

    pub strength: i32,
    pub int: i32,
    pub resolve: i32,
    pub devotion: i32,

    pub gen: rand::rngs::ThreadRng
}

impl Player {
    pub fn new() -> Player {
        let mut rng = thread_rng();
        Player {
            health: 10,
            max_health: 10,
            exp: 0,
            level: 1,
            weapon: Weapon {name: "Fists", damage: 0, speed: 1, crit: 2, atk_txt: "punch"},
            armor: Armor {name: "Cloth Tunic", value: 0, magic_res: 0},
            strength: 3,
            int: 3,
            resolve: 2,
            devotion: 2,
            gen: rng
        }
    }
    pub fn attack(&mut self, target: &mut Enemy) {
        let mut num_atks: i32 = target.speed / self.weapon.speed;
        if num_atks < 1 { num_atks = 1 };
        let mut num_atks_enemy: i32 = self.weapon.speed / target.speed;
        if num_atks_enemy < 1 {num_atks_enemy = 1}

        //Player attacks
        for _ in 0..num_atks {
            let dmg_amt = self.weapon.damage + self.gen.gen_range(1, self.strength) - target.armor;

            if self.weapon.crit >= self.gen.gen_range(1, 101) {
                target.health -= dmg_amt * 2;
                println!("you crit the {} with your {} for {} damage! nice", target.name, self.weapon.name, dmg_amt * 2);
            } else {
                target.health -= dmg_amt;
                println!("you {} the {} with your {} for {} damage", self.weapon.atk_txt, target.name, self.weapon.name, dmg_amt);
            }
        }
        println!("");
        if target.health <= 0 {
            return 
        }

        //enemy retaliation
        target.attack(num_atks_enemy, self);
    }

    pub fn display_stats(&self) {
        println!("
        Level     {} \n
        Health    {}/{} \n 
        Exp       {} \n
        Strength  {} \n
        Intellect {} \n
        Devotion  {} \n
        Resolve   {} \n
        ", self.level, self.health, self.max_health, self.exp, self.strength,
           self.int, self.devotion, self.resolve)
    }

    pub fn level_up(&mut self, mut choice: i32, random: i32, is_special: bool) {
        //Special circumstances include start of game stats and items, otherwise level normally
        if !is_special {
            self.level += 1;
            self.max_health += 5;
            self.health = self.max_health;
        }

        for _ in 0..random {
            match self.gen.gen_range(0,4) {
                0 => {self.strength += 1; println!("Your stength increases by 1")},
                1 => {self.int += 1; println!("Your intellect increases by 1")},
                2 => {self.devotion += 1; println!("Your devotion increases by 1")},
                3 => {self.resolve += 1; println!("Your resolve increases by 1")},
                _ => {},
            }
        }

        self.display_stats();
        println!("At any point you can enter 'help' to get information on commands");
        println!("enter the name of the stat you want to increase: strength, intellect, devotion, resolve");
        while choice > 0 {

            let mut input = String::new();
            println!("stat increases remaining: {}", choice);
            stdin().read_line(&mut input).expect("Input error");
            let cleaned = input.trim().to_lowercase();
            let mut commands = cleaned.split_whitespace();

            match commands.next() {
                Some("strength") => {self.strength += 1; println!("Your stength increases by 1"); choice -= 1},
                Some("intellect") => {self.int += 1; println!("Your intellect increases by 1"); choice -= 1},
                Some("devotion") => {self.devotion += 1; println!("Your devotion increases by 1"); choice -= 1},
                Some("resolve") => {self.resolve += 1; println!("Your resolve increases by 1"); choice -= 1},
                Some("help") => println!("To incearse a stat type the name of it and press enter.
                Strength - increases damage done by phsycial attacks
                Intellect - increases damage done by spells
                Devotion - increase your chance of getting spells and boons
                Resolve - you heal by your resolve amount at the end of combat, some enemies or items may test your resolve"),

                None => {println!("Please type either strength, intellect, devotion or resolove")},
                _ => {println!("Please type either strength, intellect, devotion or resolove")},
            }
        }
    }

    pub fn display_inventory(&self) {
        println!("--------- equipped ---------");
        println!("Weapon: {} - dmg {} spd {} crit {}%", self.weapon.name, self.weapon.damage, self.weapon.speed, self.weapon.crit);
        println!("Armor: {} - armor {} magic {}", self.armor.name, self.armor.value, self.armor.magic_res);
        println!("----------------------------")
    }
}