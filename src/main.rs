use std::env;

fn main() {
    println!("Almighty formula by Hossana");

    let args: Vec<String> = env::args().collect();
    let mut parameters: Vec<f64> = Vec::new();

    for arg in &args[1..] {
        match arg.parse::<f64>() {
            Ok(value) => parameters.push(value),
            Err(e) => {
                eprintln!("Error parsing {} arg as f64: {}", arg, e);
                std::process::exit(1);
            }
        }
    }

    if parameters.len() < 3 {
        eprintln!("Not enough arguments provided.");
        std::process::exit(1);
    }

    if parameters.len() > 3 {
        eprintln!("Arguments provided is more than expected.");
        std::process::exit(1);
    }

    let a: f64 = parameters[0];
    let b: f64 = parameters[1];
    let c: f64 = parameters[2];

    let discriminant: f64 = b.powf(2.0) - 4.0 * a * c;

    if a == 0.0 {
        eprintln!("'a' cannot be zero.");
        std::process::exit(1);
    } else if discriminant < 0.0 {
        println!("No real roots exist.");
    } else {
        let numerator1: f64 = -b + discriminant.sqrt();
        let numerator2: f64 = -b - discriminant.sqrt();
        let denominator: f64 = 2.0 * a;

        let answer1: f64 = numerator1 / denominator;
        let answer2: f64 = numerator2 / denominator;

        println!("If a is {}, b is {}, and c is {}, the answers is: {} or {}", a, b, c, answer1, answer2);
    }
}
