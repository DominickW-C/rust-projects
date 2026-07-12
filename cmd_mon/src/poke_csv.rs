use std:: {
    io::prelude::*,
    fs
};
use crate::TEAM;
use crate::poke;

//csv functions for dex

//csv functions for team

/*
 * looks through provided dex to find Pokemon matching new_mon
 * returns "e" on error
 * returns pokemon string on success
 */
pub fn add_string_to_team_file(new_mon: String, dex: &str) -> &str{
    let mut t_file = match fs::OpenOptions::new()
        .append(true)
        .open(TEAM) {
            Ok(r) => r,
            Err(e) => {
                println!("problem reading the team file {e}");
                return "e"
            }
        };

    println!("finding new pokemon: {new_mon}");
    for entry in dex.lines() {
        if entry.contains(&new_mon) {
            t_file.write_all(entry.as_bytes()).expect("Problem writing to team file");
            t_file.write_all("\n".as_bytes()).expect("Problem writing to team file");
            return entry;
        }
    }
    "e"
}

pub fn add_struct_to_team_file(new_mon: &poke::Pokemon) -> String {
    let mut t_file = match fs::OpenOptions::new()
        .append(true)
        .open(TEAM) {
            Ok(r) => r,
            Err(e) => {
                println!("problem reading the team file {e}");
                return "e".to_string()
            }
        };

    let hp: String = new_mon.hp.to_string();
    let at: String = new_mon.attack.to_string();
    let def: String = new_mon.defense.to_string();
    let sp_a: String = new_mon.sp_attack.to_string();
    let sp_d: String = new_mon.sp_def.to_string();
    let sp: String = new_mon.speed.to_string();
    let c_r: String = new_mon.catch_rate.to_string();
    let mon_string: String = new_mon.name.clone() + "," 
        + &hp + "," + &at + "," + &def + ","+ &sp_a + "," 
        + &sp_d + "," + &sp + "," + &c_r + "\n";
    t_file.write_all(mon_string.as_bytes()).expect("Problem writing to team file");
    mon_string
}
