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

    //parse args to float
    let num1 = args[1].parse::<f64>();
    let num2 = args[3].parse::<f64>();

    //check first num valid
    match num1 {
        Ok(aa) => {
            //check second num valid
            match num2 {
                Ok(bb) => {
                    //both numbers valid, check operator and do correct equation
                    let result: Option<f64> = do_math(aa, bb, &args[2]);
                    //if operator was valid, result was returned and print it
                    if let Some(xx) = result {
                        println!("-> {} {} {} = {}", aa, args[2], bb, xx);
                    }
                },
                Err(_e) => println!("error with first number") 
            };
        },
        Err(_e) => println!("error with second number") 
    }
}
