use std::{
    io,
    io::prelude::*,
    fs,
};

const POKEDEX: &str = "src/pokeInfo/pokedex.csv";
const TEAM: &str = "src/pokeInfo/team.csv";

struct Pokemon {
    current_hp: i32,
    name: String,
    hp: i16,
    attack: i16,
    defense: i16,
    sp_attack: i16,
    sp_def: i16,
    speed: i16
}

fn con_mon(mon_data: &[&str]) -> Pokemon {
    let hp = match mon_data[1].parse::<i16>() {
        Ok(r) => r,
        Err(e) => {
            println!("trouble parsing hp to int {e}");
            0
        }
    };
    let attack = match mon_data[2].parse::<i16>() {
        Ok(r) => r,
        Err(e) => {
            println!("trouble parsing attack to int {e}");
            0
        }
    };
    let defense = match mon_data[3].parse::<i16>() {
        Ok(r) => r,
        Err(e) => {
            println!("trouble parsing defense to int {e}");
            0
        }
    };
    let sp_attack = match mon_data[4].parse::<i16>() {
        Ok(r) => r,
        Err(e) => {
            println!("trouble parsing sp_attack to int {e}");
            0
        }
    };
    let sp_def = match mon_data[5].parse::<i16>() {
        Ok(r) => r,
        Err(e) => {
            println!("trouble parsing sp_def to int {e}");
            0
        }
    };
    let speed = match mon_data[6].parse::<i16>() {
        Ok(r) => r,
        Err(e) => {
            println!("trouble parsing speed to int {e}");
            0
        }
    };

    Pokemon {
        current_hp: hp as i32,
        name: mon_data[0].to_string(),
        hp,
        attack,
        defense,
        sp_attack,
        sp_def,
        speed
    }
}

fn add_to_team(new_mon: String, dex: &str) {
    let mut t_file = match fs::OpenOptions::new()
        .append(true)
        .open(TEAM) {
            Ok(r) => r,
            Err(e) => {
                println!("problem reading the team file {e}");
                return
            }
        };

    println!("finding new pokemon: {new_mon}");
    for entry in dex.lines() {
        if entry.contains(&new_mon) {
            t_file.write_all(entry.as_bytes()).expect("problem writing to file");
        }
    }
}

fn read_input(buf: &mut String) {
    buf.clear();
    match io::stdin().read_line(buf) {
        Ok(n) => println!("read {n} bytes"),
        Err(e) => println!("got {e}")
    }
}

fn main() {
    let mut buffer = String::new();
    let mut team: Vec<Pokemon> = Vec::new();
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

    if team_file.lines().count() == 1 {
        println!("team is empty, need to choose a pokemon");
        println!("1. Squirtle");
        println!("2. Piplup");
        read_input(&mut buffer);
        println!("{buffer}");
        if buffer == "1\n" {
            println!("you chose Squirtle");
            add_to_team("Squirtle".to_string(), &dex);
        } else if buffer == "2\n" {
            println!("you chose Piplip");
            add_to_team("Piplup".to_string(), &dex);
        } else {
            println!("error, please input valid number");
        }
    } 

    //let dex_entries = dex.lines(); 

    //TODO: skip the first entry
    for entry in dex.lines() {
        println!("test {entry}");
        let spl: Vec<&str> = entry.split(',').collect();
        let mon: Pokemon = con_mon(&spl);
        team.push(mon);
        println!("split result: {}", spl[0]);
    } 

    println!("after adding to team, team file is");
    println!("{}", fs::read_to_string(TEAM).expect("should be 2 lines"));

    println!("your team consists of");
    for mon in team {
        println!("{}", mon.name);
        println!("Stats:");
        println!("current hp: {}", mon.current_hp);
        println!("hp {}", mon.hp);
        println!("attack {}", mon.attack);
        println!("defense {}", mon.defense);
        println!("sp_attack {}", mon.sp_attack);
        println!("sp_def {}", mon.sp_def);
        println!("speed {}", mon.speed);
    }

}
