pub fn valid_guesses() -> Vec<String> {
    let file_data = include_str!("./guess.txt");
    let words = file_data.split("\n");

    words.map(|x| x.to_string()).collect()
}

pub fn valid_wordles() -> Vec<String> {
    let file_data = include_str!("./answer.txt");
    let words = file_data.split("\n");

    words.map(|x| x.to_string()).collect()
}
