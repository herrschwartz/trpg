use crate::floors::get_single_command;

pub fn tutorial() {
    println!("1) New Game");
    println!("2) Tutorial");
    let cmd = get_single_command();
    let choice = cmd.as_str();
    match choice {
        "1" => return,
        "2" => println!("Ok, lets get into it"),
        _ => println!("Please type the number of what you would like to do, then press Enter"),
    }
}