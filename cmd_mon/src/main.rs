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
        team.push(poke::con_mon(mon_string));
    }
}

fn main() {
    let mut buffer = String::new();
    let mut team: Vec<poke::Pokemon> = Vec::new();
    let mut opponent: Vec<poke::Pokemon> = Vec::new();

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
                let starter: &str = poke_csv::add_string_to_team_file("Squirtle".to_string(), &dex);
                add_starter_to_team(starter, &mut team);
                break;
            } else if buffer == "2\n" {
                println!("You picked Piplup as your starter");
                let starter: &str = poke_csv::add_string_to_team_file("Piplup".to_string(), &dex);
                add_starter_to_team(starter, &mut team);
                break;
            } else if buffer == "3\n" {
                println!("Exiting the game...");
                return
            } else {
                println!("Error, please input valid number");
            }
        }
    } else {
        for (num, file_mon) in (0_u8..).zip(team_file.lines()) {
            if num == 0 {
                continue;
            }
            team.push(poke::con_mon(file_mon));
        }
    }

    //main menu loop
    loop {
        println!("----------------------------");
        println!("What would you like to do?");
        println!("-> 1. View Team");
        println!("-> 2. Wild Battle");
        println!("-> 3. Exit");
        read_input(&mut buffer);
        //change to match case if possible
        if buffer == "1\n" {
            poke::print_team(&team);
        } else if buffer == "2\n" {
            //0-1025 + 1 to account for csv header
            let rand_num: u16 = (rand::random::<u16>() % 1026) + 1;
            println!("random numbr was {rand_num}"); 
            match dex.lines().nth(rand_num as usize) {
                Some(wild_mon) => {
                    opponent.push(poke::con_mon(wild_mon))
                }
                None => {
                    println!("Problem generating a wild Pokemon");
                    return
                }
            };

            loop {
                println!("----------------------------");
                println!("Opponent:");
                println!("{}", opponent[0].name);
                println!("HP: {}/{}", opponent[0].current_hp, opponent[0].hp);
                println!("***********");
                println!("Your Pokemon:");
                println!("{}", team[0].name);
                println!("HP: {}/{}", team[0].current_hp, team[0].hp);
                println!("----------------------------");
                println!("What would you like to do?");
                println!("1. Attack");
                println!("2. Catch");
                println!("3. Run");
                read_input(&mut buffer);
                if buffer == "1\n" {
                    //TODO: implement battle
                } else if buffer == "2\n" {
                    //finds a in shake prob equation
                    let p1: f64 = 3.0 * opponent[0].hp as f64;
                    let p2: f64 = 2.0 * opponent[0].current_hp as f64;
                    let p3: f64 = p1 - p2;
                    let p4: f64 = p3 / p1;
                    let p5: f64 = p4 * opponent[0].catch_rate as f64;

                    //solves for b in shake prob equation
                    let p6: f64 = 16711680.0 / p5;
                    let p7: f64 = p6.sqrt().sqrt();
                    let p8: f64 = 1048560.0 / p7;

                    //check four shakes to see if caught
                    let mut shake_count: u16 = 0;
                    while shake_count < 4 {
                        let shake_random = rand::random::<u32>() % 65536;
                        if shake_random >= p8 as u32 {
                            println!("Catch failed");
                            break;
                        } else {
                            shake_count += 1;
                        }
                    }
                    
                    //successful catch, add to your team file and team vec
                    if shake_count == 4 {
                        println!("Caught {}!", opponent[0].name);
                        let new_mon: poke::Pokemon = poke::Pokemon {
                            current_hp : opponent[0].current_hp,
                            name: opponent[0].name.clone(),
                            hp: opponent[0].hp,
                            attack: opponent[0].attack,
                            defense: opponent[0].defense,
                            sp_attack: opponent[0].sp_attack,
                            sp_def: opponent[0].sp_def,
                            speed: opponent[0].speed,
                            catch_rate: opponent[0].catch_rate
                        };
                        //TODO: fix error handling
                        let add_result: String = poke_csv::add_struct_to_team_file(&new_mon);
                        if add_result == "e" {
                            println!("Could not add the pokemon to your team");
                        } else {
                            team.push(new_mon);
                        }
                        opponent.pop();
                        break;
                    }
                } else if buffer == "3\n" {
                    opponent.pop();
                    break;
                } else {
                    println!("Please input a valid number");
                }
            }
        } else if buffer == "3\n" {
            break;
        } else {
            println!("Please input a valid number");
        }
    };
}
