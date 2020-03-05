use crate::items::Weapon;
use crate:: items::Spell;
use crate::enemys::Enemy;
use crate::LEVELS;
use rand::prelude::*;
use std::io::stdin;
use std::{thread, time};


pub struct Player {
    pub health: i32,
    pub max_health: i32,
    pub armor: i32,
    pub armor_magic: i32,
    pub lifeforce: i32,
    pub exp: i32,
    pub level: i32,
    pub weapon: Weapon,
    pub spells: Vec<Spell>,
    pub weapons: Vec<Weapon>,

    pub strength: i32,
    pub int: i32,
    pub resolve: i32,
    pub devotion: i32,

    pub gen: rand::rngs::ThreadRng
}

impl Player {
    pub fn new() -> Player {
        let rng = thread_rng();
        Player {
            health: 10,
            max_health: 10,
            exp: 0,
            level: 1,
            lifeforce: 0,
            weapon: Weapon {name: "Fists", damage: 0, speed: 1, crit: 1, atk_txt: "punch", crit_txt: "smash", rank: 0},
            armor: 0,
            armor_magic: 0,
            spells: vec![Spell{name: "arcane bolt", description: "A basic bolt of magic energy", 
                        speed: 1, damage: 2, kind: "arcane",
                        atk_txt: "Your hands fume with perverse energies, they coalese into a dense bead"}],
            weapons: vec![],
            strength: 3,
            int: 3,
            resolve: 2,
            devotion: 2,
            gen: rng
        }
    }

    pub fn heal(&mut self, amount: i32) {
        self.health += amount;
        if self.health > self.max_health {
            self.health = self.max_health
        }
        println!("You heal for {}", amount);
    }

    pub fn attack(&mut self, target: &mut Enemy) {
        let mut num_atks: i32 = target.speed / self.weapon.speed;
        if num_atks < 1 { num_atks = 1 };

        //Player attacks
        for _ in 0..num_atks {
            let dmg_amt = self.weapon.damage + self.gen.gen_range(1, self.strength) - target.armor;

            if self.weapon.crit >= self.gen.gen_range(1, 101) {
                target.health -= dmg_amt * 2;
                println!("you {} the {} with your {} for {} damage!!", self.weapon.crit_txt, target.name, self.weapon.name, dmg_amt * 2);
            } else {
                target.health -= dmg_amt;
                println!("you {} the {} with your {} for {} damage", self.weapon.atk_txt, target.name, self.weapon.name, dmg_amt);
            }
            thread::sleep(time::Duration::from_millis(600));
        }
        println!();
        if target.health <= 0 {
            return 
        }

        //enemy retaliation
        thread::sleep(time::Duration::from_millis(600));
        target.attack(self.weapon.speed, self);
    }

    pub fn cast_spell(&mut self, target: &mut Enemy, spell_name: String) {
        let spell_pos = self.spells.iter().position(|x| x.name == spell_name); 
        match spell_pos {
            Some(spell_pos) => {
                let spell = &self.spells[spell_pos];
                let mut num_atks_enemy: i32 = spell.speed / target.speed;
                if num_atks_enemy < 1 {num_atks_enemy = 1}

                let mut dmg = match spell.kind {
                    "arcane"    => spell.damage + self.int - target.magic_res,
                    "lightning" => spell.damage + self.gen.gen_range(self.int/2, self.int + 1) + self.gen.gen_range(0, self.int) - target.magic_res,
                    "fire"      => spell.damage - target.magic_res,
                    _ => panic!("invalid spell type")
                };
                if dmg < 0 {dmg = 0}
                target.health -= dmg;
                println!("{} and you hit the {} for {}", spell.atk_txt, target.name, dmg);

                println!();
                if target.health <= 0 {
                    return 
                }
                //enemy attacks back
                target.attack(spell.speed, self);

                //remove used spell from inventory
                self.spells.remove(spell_pos);
            },
            None => println!("You don't have {}", spell_name)
        }
    }

    pub fn invoke(&mut self, target: &mut Enemy, spell_name: String, in_combat: bool) {
        
    }
    
    pub fn display_stats(&self) {
        println!("
        Level     {} \n
        Health    {}/{} \n 
        Exp       {}/{} \n
        Strength  {} \n
        Intellect {} \n
        Devotion  {} \n
        Resolve   {} \n
        ", self.level, self.health, self.max_health, self.exp, LEVELS[(self.level - 1) as usize], 
         self.strength, self.int, self.devotion, self.resolve)
    }

    pub fn level_up(&mut self, mut choice: i32, random: i32, is_special: bool) {
        //Special circumstances include start of game, and items. otherwise level normally
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
                Some("help") => println!("To improve a stat type the name of the stat and press enter.
                Strength - increases damage done by phsycial attacks
                Intellect - increases damage done by spells
                Devotion - increase your chance of getting spells and blessings
                Resolve - you heal by your resolve amount at the end of combat, some enemies or items may test your resolve"),

                None => {println!("Please type either strength, intellect, devotion or resolove")},
                _ => {println!("Please type either strength, intellect, devotion or resolove")},
            }
        }
    }

    pub fn display_inventory(&self) {
        println!("\nLifeforce: {}", self.lifeforce);
        println!("--------- equipped ---------");
        println!("Weapon: {} - dmg {} spd {} crit {}%", self.weapon.name, self.weapon.damage, self.weapon.speed, self.weapon.crit);
        println!("Armor: Physical {} magic {}", self.armor, self.armor_magic);
        println!("---------------------------- \n");
        for s in &self.spells {
            println!("{} - {}", s.name, s.description);
        }
        for t in &self.weapons {
            println!("{} - dmg {} spd {} crit {}%", t.name, t.damage, t.speed, t.crit);
        }
        println!();
    }

    pub fn equip(&mut self, weapon_name: String) {
        let weapon = self.weapons.iter().position(|x| x.name.to_lowercase() == weapon_name);
        match weapon {
            Some(weapon) => {
                self.weapons.push(self.weapon.clone());
                self.weapon = self.weapons[weapon].clone();
                self.weapons.remove(weapon);
            }
            None => println!("You don't have a weapon in your inventory named {}", weapon_name)
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}