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
