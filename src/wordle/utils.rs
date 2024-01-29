use super::model::{LetterState, LetterStatus};

// wordle is compared with incoming string and each character is parsed for its correctness.
// There are three cases - correct position, incorrect position, unknown (not present etc)
// CASE 1: Correct position - letter matches exact the position of the wordle
// CASE 2: Incorrect position
// First we have to check if the letter is present anywhere else in the wordle. And an additional
// check has to performed for total number of those letters in the wordle and the guessed word. The
// letter is marked as incorrect if wordle has more or equal number of letters in the guessed word
// CASE 3: unknown - letter is not present in the string at all
pub fn check(wordle: String, guess: String) -> Vec<LetterStatus> {
    let mut output: Vec<LetterStatus> = Vec::with_capacity(5);
    // first things first; let use make sure we have the right length words
    if wordle.len() != 5 && guess.len() != 5 {
        panic!("5 letter wordle and guess word is needed. Aborting");
    }

    let wordle_letters: Vec<char> = wordle.chars().collect();
    let guess_letters: Vec<char> = guess.chars().collect();

    for position in 0..5 {
        let wordle_letter = wordle_letters[position];
        let guess_letter = guess_letters[position];
        // CASE 1: correct position
        if wordle_letter == guess_letter {
            output.push(LetterStatus {
                letter: guess_letter,
                status: LetterState::Correct,
            })
        } else {
            // CASE 3: guess letter is not present in wordle
            if !wordle_letters.contains(&guess_letter) {
                output.push(LetterStatus {
                    letter: guess_letter,
                    status: LetterState::Unknown,
                })
            } else {
                // CASE 2: letter is present but not in the right position
                // first let us find character occurences in the wordle
                let wordle_occurences =
                    wordle_letters.clone().into_iter().fold(0, |acc, letter| {
                        if letter == guess_letter {
                            acc + 1
                        } else {
                            acc
                        }
                    });

                let guess_occurences = guess_letters.clone().into_iter().fold(0, |acc, letter| {
                    if letter == guess_letter {
                        acc + 1
                    } else {
                        acc
                    }
                });

                // wordle occurences should be greater than or equal to guess letter occurences
                if wordle_occurences >= guess_occurences {
                    output.push(LetterStatus {
                        letter: guess_letter,
                        status: LetterState::Incorrect,
                    })
                } else {
                    output.push(LetterStatus {
                        letter: guess_letter,
                        status: LetterState::Unknown,
                    })
                }
            }
        }
    }

    output
}

pub fn is_correct_guess(input: Vec<LetterStatus>) -> bool {
    input.iter().all(|x| x.status == LetterState::Correct)
}

#[cfg(test)]
mod tests {
    use crate::wordle::model::{LetterState, LetterStatus};
    use crate::wordle::utils::*;

    #[test]
    fn test_check() {
        let output = check("helio".into(), "hello".into());

        let expected: Vec<LetterStatus> = vec![
            LetterStatus {
                letter: 'h',
                status: LetterState::Correct,
            },
            LetterStatus {
                letter: 'e',
                status: LetterState::Correct,
            },
            LetterStatus {
                letter: 'l',
                status: LetterState::Correct,
            },
            LetterStatus {
                letter: 'l',
                status: LetterState::Unknown,
            },
            LetterStatus {
                letter: 'o',
                status: LetterState::Correct,
            },
        ];

        assert_eq!(output, expected);
    }

    #[test]
    fn test_letter_status() {
        let all_correct_input: Vec<LetterStatus> = vec![
            LetterStatus {
                letter: 'p',
                status: LetterState::Correct,
            },
            LetterStatus {
                letter: 'i',
                status: LetterState::Correct,
            },
            LetterStatus {
                letter: 'o',
                status: LetterState::Correct,
            },
            LetterStatus {
                letter: 'u',
                status: LetterState::Correct,
            },
            LetterStatus {
                letter: 's',
                status: LetterState::Correct,
            },
        ];

        assert!(is_correct_guess(all_correct_input));

        let not_correct_input: Vec<LetterStatus> = vec![
            LetterStatus {
                letter: 'x',
                status: LetterState::Unknown,
            },
            LetterStatus {
                letter: 'i',
                status: LetterState::Correct,
            },
            LetterStatus {
                letter: 'o',
                status: LetterState::Correct,
            },
            LetterStatus {
                letter: 'u',
                status: LetterState::Correct,
            },
            LetterStatus {
                letter: 's',
                status: LetterState::Correct,
            },
        ];

        assert_eq!(is_correct_guess(not_correct_input), false);
    }
}
