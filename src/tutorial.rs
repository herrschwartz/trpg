use crate::floors::get_single_command;
use std::{thread, time};
use std::io::stdout;
use std::io::Write;

pub fn tutorial() {
    println!("1) New Game");
    println!("2) Tutorial");
    let mut brk = false;
    loop {
        let cmd = get_single_command();
        let choice = cmd.as_str();
        match choice {
            "1" => return,
            "2" => brk = tutorial_intro(),
            _ => println!("Please type the number of what you would like to do, then press Enter"),
        }
        if brk {
            break;
        }
    }
}

fn get_next() {
    loop {
        let cmd = get_single_command();
        let choice = cmd.as_str();
        match choice {
            "next" => return,
            _ => println!("Please type 'next' and press the Enter key to continue"),
        }
    }
}

fn cool_print(text: Vec<&str>) {
    for i in text {
        for j in i.chars() {
            print!("{}", j);
            stdout().flush().unwrap();
            thread::sleep(time::Duration::from_millis(20));
        }
        println!();
    }
}

fn tutorial_intro() -> bool{
    cool_print(
        vec![
            "Welcome!",
            "In this game you will be moving through a series of rooms",
            "The rooms come in four distinct types",
            "Combat, items, shops and special events",
            "After each room you will have the chance to rest before moving on the next room",
            "To move to the next room you have to type 'next' and press the Enter key",
            "This is called entering a command",
            "Try typing 'next' and pressing the Enter key now"
        ]
    );
    get_next();
    cool_print(
        vec![
            "Commands:",
            "--------",
            "Congratulations, You've entered your first command!",
            "Commands are how you will navigate this game",
            "You will have to enter one if you want to do anything",
            "Typing the 'help' command and pressing Enter will give you",
            "A list of availible commands at any point in the game",
            "Much of the time, like in shops, you will only have Enter a number to select what you want to do",
            "Enter next to continue..."
        ]
    );
    get_next();
    cool_print(
        vec![
            "Stats:",
            "--------",
            "Your stats increase the effectiveness of differnt actions as follows:",
            "Strength  - Increases the damage you do with physical attacks",
            "Intellect - Increases the damage that most magic attacks do",
            "Devotion  - Increases the effectiveness of certain blessings like heals and also increases your chance to recieve spells and blessings when resting",
            "Resolve   - Heals you after completing combat",
            "Enter next to continue..."
        ]
    );
    get_next();
    cool_print(
        vec![
            "Weapons:",
            "--------",
            "Weapons are are used during combat only",
            "You can attack with a weapon by Entering the attack command while in combat",
            "A weapon has three attribute: attack, speed and crit",
            "Attack is how much raw damage the weapon does, but strength has a chance to increase it",
            "Speed is how fast the weapon attacks, using a fast weapon will grant you more attacks on slower enemies,",
            "But will do less damage in a single hit",
            "Crit is a percentage chance to land a critical blow",
            "Critical blows do 200% damage",
            "Enter next to continue..."
        ]
    );
    get_next();
    cool_print(
        vec![
            "Spells:",
            "--------",
            "Spells can only be used in combat",
            "To use a spell you Enter 'cast <spell name>' as a command",
            "There are three types of spell: Fire, Thunder and Arcane",
            "Fire does flat damage and does not depend on Intellect",
            "Arcane has high base damage, but scales with intellect",
            "Thunder has low base damage and deals random damage based on how much Intellect you have",
            "Enter next to continue..."
        ]
    );
    get_next();
    cool_print(
        vec![
            "Blessings:",
            "--------",
            "To use a blessing you Enter 'invoke <blessing name>' as a command",
            "Some blessings can be used out of combat when you are resting",
            "Heals are an example of this",
            "Other blessings may only be used in combats",
            "A blessing that increases your strength would be an example of this",
            "That's all! Thanks for reading and have fun! Enter next to finish..."
        ]
    );
    get_next();
    true
}