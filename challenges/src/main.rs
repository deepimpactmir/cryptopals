use std::collections::HashMap;
use std::env;
use std::error::Error;

mod set1;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please specify a challenge number. Example: `cargo run 1`");
        return;
    }

    // Parse the provided argument to a u32 (challenge number)
    let challenge_number = match args[1].parse::<u32>() {
        Ok(number) => number,
        Err(_) => {
            println!(
                "Invalid challenge number. Please specify a valid u32. Example: `cargo run 1`"
            );
            return;
        }
    };

    // Register all challenges by calling set1::add_challenges(challenge_map);
    let mut challenge_map: HashMap<u32, fn() -> Result<(), Box<dyn Error>>> = HashMap::new();
    set1::add_challenges(&mut challenge_map);

    // Dispatch to the appropriate challenge
    match challenge_map.get(&challenge_number) {
        Some(challenge) => {
            if let Err(e) = challenge() {
                println!("Error running challenge {}: {}", challenge_number, e);
            }
        }
        None => println!("Challenge {} not implemented.", challenge_number),
    }
}
