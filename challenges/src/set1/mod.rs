use std::collections::HashMap;
use std::error::Error;

mod challenge01;

pub fn add_challenges(challenge_map: &mut HashMap<u32, fn() -> Result<(), Box<dyn Error>>>) {
    challenge_map.insert(1, challenge01::run);
}
