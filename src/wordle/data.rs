use std::fs;

fn parse_file(path: String) -> Vec<String> {
    let file_data = fs::read_to_string(path.clone());

    if file_data.is_err() {
        panic!("cannot read file {}", path.clone());
    }

    let file_data = file_data.unwrap();
    let words = file_data.split("\n");

    words.map(|x| x.to_string()).collect()
}

pub fn valid_guesses() -> Vec<String> {
    parse_file("./data/guess.txt".to_string())
}

pub fn valid_wordles() -> Vec<String> {
    parse_file("./data/answer.txt".to_string())
}
