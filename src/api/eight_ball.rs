//! Internal API for the "Magic 8-ball".

use anyhow::Context;
use rand::Rng;

const EIGHTBALL_RESPONSES: [&str; 20] = [
    "It is certain",
    "It is decidedly so",
    "Without a doubt",
    "Yes, definitely",
    "You may rely on it",
    "As I see it, yes",
    "Most likely",
    "Outlook good",
    "Yes",
    "Signs point to yes",
    "Reply hazy try again",
    "Ask again later",
    "Better not tell you now",
    "Cannot predict now",
    "Concentrate and ask again",
    "Don't count on it",
    "My reply is no",
    "My sources say no",
    "Outlook not so good",
    "Very doubtful",
];

/// Get a random response from the
pub fn get_random_response() -> anyhow::Result<&'static str> {
    let rand_num = rand::thread_rng().gen_range(0..EIGHTBALL_RESPONSES.len());
    EIGHTBALL_RESPONSES
        .get(rand_num)
        .context("failed to get random 8-ball response")
        .copied()
}
