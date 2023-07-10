//! Internal API for the "Magic 8-ball".

use rand::Rng;

/// Get a random response from the
pub fn get_random_roll(num: u64) -> u64 {
    rand::thread_rng().gen_range(0..=num)
}
