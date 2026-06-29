use std::{
    io,
    io::Write,
    fs,
};
mod poke;
mod poke_csv;

const POKEDEX: &str = "src/pokeInfo/pokedex.csv";
pub const TEAM: &str = "src/pokeInfo/team.csv";

fn read_input(buf: &mut String) {
    buf.clear();
    print!("Please input a number > ");
    io::stdout().flush().unwrap();
    match io::stdin().read_line(buf) {
        Ok(_) => (),
        Err(e) => println!("Error reading input: {e}")
    }
}

fn add_starter_to_team(mon_string: &str, team: &mut Vec<poke::Pokemon>) {
    if mon_string != "e" {
        let spli: Vec<&str> = mon_string.split(',').collect();
        let mon: poke::Pokemon = poke::con_mon(&spli);
        team.push(mon);
    }
}

fn main() {
    let mut buffer = String::new();
    let mut team: Vec<poke::Pokemon> = Vec::new();

    let dex = match fs::read_to_string(POKEDEX) { 
        Ok(r) => r,
        Err(e) => {
            println!("problem reading the dex {e}");
            return
        }
    };

    let team_file = match fs::read_to_string(TEAM) {
        Ok(r) => r,
        Err(e) => {
            println!("problem reading team file {e}");
            return
        }
    };

    //game init
    //true: new game select a starter
    //false: continue game, load into team vec
    if team_file.lines().count() == 1 {
        loop {
            println!("----------------------------");
            println!("Team is empty! Choose a starter:");
            println!("-> 1. Squirtle");
            println!("-> 2. Piplup");
            println!("-> 3. Exit");
            read_input(&mut buffer);
            if buffer == "1\n" {
                println!("You picked Squirtle as your starter");
                let starter: &str = poke_csv::add_to_team_file("Squirtle".to_string(), &dex);
                add_starter_to_team(starter, &mut team);
                break;
            } else if buffer == "2\n" {
                println!("You picked Piplup as your starter");
                let starter: &str = poke_csv::add_to_team_file("Piplup".to_string(), &dex);
                add_starter_to_team(starter, &mut team);
                break;
            } else if buffer == "3\n" {
                println!("Exiting the game...");
                return
            } else {
                println!("error, please input valid number");
            }
        }
    } else {
        for (num, file_mon) in (0_u8..).zip(team_file.lines()) {
            if num == 0 {
                continue;
            }
            let spli: Vec<&str> = file_mon.split(',').collect();
            let mon: poke::Pokemon = poke::con_mon(&spli); 
            team.push(mon);
        }
    }

    //main menu loop
    loop {
        println!("----------------------------");
        println!("What would you like to do?");
        println!("-> 1. view team");
        println!("-> 2. exit");
        read_input(&mut buffer);
        //change to match case if possible
        if buffer == "1\n" {
            poke::print_team(&team);
        } else if buffer == "2\n" {
            break;
        } else {
            println!("Please input a valid number");
        }
    };
}
