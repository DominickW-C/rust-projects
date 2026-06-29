pub struct Pokemon {
    pub current_hp: i32,
    pub name: String,
    pub hp: i16,
    pub attack: i16,
    pub defense: i16,
    pub sp_attack: i16,
    pub sp_def: i16,
    pub speed: i16
}

pub fn print_team(team: &[Pokemon]) {
    for (num, mon) in (1_u8..).zip(team.iter()) {
        println!("----------------------------");
        println!("Pokemon {num}: {}", mon.name);
        println!("Current HP: {}/{}", mon.current_hp, mon.hp);
        println!("HP: {}", mon.hp);
        println!("Attack: {}", mon.attack);
        println!("Defense: {}", mon.defense);
        println!("SP Attack: {}", mon.sp_attack);
        println!("SP Defense: {}", mon.sp_def);
        println!("Speed: {}", mon.speed);
    }
}

pub fn con_mon(mon_data: &[&str]) -> Pokemon {
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
