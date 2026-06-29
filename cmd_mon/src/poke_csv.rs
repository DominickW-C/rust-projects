use std:: {
    io::prelude::*,
    fs
};
use crate::TEAM;

//csv functions for dex

//csv functions for team

/*
 * looks through provided dex to find Pokemon matching new_mon
 * returns "e" on error
 * returns pokemon string on success
 */
pub fn add_to_team_file(new_mon: String, dex: &str) -> &str{
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
            return entry;
        }
    }
    "e"
}
