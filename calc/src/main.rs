use std::env;
mod math;

fn do_math(n1: f64, n2: f64, op: &String) -> Option<f64> {
    if op == "+" {
        return Some(math::add(n1, n2));
    }
    if op == "-" {
        return Some(math::sub(n1, n2));
    }
    if op == "*" {
        return Some(math::mul(n1, n2));
    }
    if op == "/" {
        return Some(math::div(n1, n2));
    }
    println!("invalid operator, please use +, -, *, /");
    None
}

fn main() {
    //pull the command line arguements
    let args: Vec<String> = env::args().collect();
    
    //check for valid math function
    if args.len() <= 3 {
        println!("Please input a valid math function"); 
        return
    }

    //checks that num1 is valid
    let num1 = match args[1].parse::<f64>() {
        Ok(r) => r,
        Err(_e) => {
            println!("invalid number for first number");
            return
        }
    };

    //checks that num2 is valid
    let num2 = match args[3].parse::<f64>() {
        Ok(r) => r,
        Err(_e) => {
            println!("invalid number for second number");
            return
        } 
    };

    //both numbers valid, check operator and do correct equation
    let result: Option<f64> = do_math(num1, num2, &args[2]);
    //if operator was valid, result was returned and print it
    if let Some(xx) = result {
        println!("-> {} {} {} = {}", num1, args[2], num2, xx);
    }
}
