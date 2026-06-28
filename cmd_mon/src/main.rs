use std::{
    io,
    fs,
    env
};

const POKEDEX: &str = "src/pokeInfo/pokedex.csv";

/*
struct Pokemon {
    current_hp: i32,
    hp: i16,
    attack: i16,
    defense: i16,
    sp_attack: i16,
    sp_def: i16,
    speed: i16
}
*/

fn read_input(buf: &mut String) {
    buf.clear();
    match io::stdin().read_line(buf) {
        Ok(n) => println!("read {n} bytes"),
        Err(e) => println!("got {e}")
    }
}

fn main() {
    let path = match env::current_dir() {
        Ok(n) => n,
        Err(_) => {
            println!("how could I get an error here"); 
            return
        }
    };
    println!("{}", path.display());
    let dex = match fs::read_to_string(POKEDEX) { 
        Ok(r) => r,
        Err(e) => {
            println!("problem reading the dex {e}");
            return
        }
    };
    
    //let dex_entries = dex.lines(); 

    for entry in dex.lines() {
        println!("test {entry}");
    } 
    

    println!("Hello, world!");
    let mut buffer = String::new();
    read_input(&mut buffer);
    println!("input something");
    println!("got {buffer}");
    println!("input something new");
    read_input(&mut buffer);
    println!("got {buffer}");
}
