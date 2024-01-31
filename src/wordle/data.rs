use rand::seq::SliceRandom;

pub fn valid_guesses() -> Vec<String> {
    vec![
        "hello", "piano", "pious", "great", "alone", "music", "abide",
    ]
    .iter()
    .map(|x| x.to_string())
    .collect()
}

pub fn valid_wordles() -> Vec<String> {
    vec!["pious", "great"]
        .iter()
        .map(|x| x.to_string())
        .collect()
}

pub fn get_wordle() -> Option<String> {
    let guesses = valid_guesses();
    if let Some(guess) = guesses.choose(&mut rand::thread_rng()) {
        Some(guess.to_string())
    } else {
        None
    }
}
