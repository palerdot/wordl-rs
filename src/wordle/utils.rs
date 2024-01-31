use super::model::{KeyboardHints, LetterState, LetterStatus};

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
                    status: LetterState::NotPresent,
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
                        status: LetterState::NotPresent,
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

// helper function to update keyboard hints
pub fn update_keyboard_hints<'a>(
    hints: &'a mut KeyboardHints,
    statuses: Vec<LetterStatus>,
) -> &'a mut KeyboardHints {
    for status in statuses {
        // if the key is not present; just update with the value
        if !hints.contains_key(&status.letter) {
            hints.insert(status.letter.clone(), status.status.clone());
        } else {
            // key is present; based on the status we have to match stuff
            match status.status {
                LetterState::Unknown | LetterState::NotPresent | LetterState::Correct => {
                    hints.insert(status.letter.clone(), status.status.clone());
                }
                LetterState::Incorrect => {
                    // incorrect is a special case; if the existing value is already "correct" we
                    // have to leave as is; else just update the value
                    if let Some(current_status) = hints.get(&status.letter) {
                        if *current_status != LetterState::Correct {
                            hints.insert(status.letter.clone(), status.status.clone());
                        }
                    }
                }
            }
        }
    }

    hints
}

#[cfg(test)]
mod tests {
    use crate::wordle::model::{LetterState, LetterStatus};
    use crate::wordle::utils::*;
    use std::collections::HashMap;

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
                status: LetterState::NotPresent,
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

    #[test]
    fn test_keyboard_hints() {
        // WORDLE - PIOUS
        let wordle = String::from("pious");
        let mut hints: KeyboardHints = HashMap::new();
        let statuses: Vec<LetterStatus> = check(wordle.clone(), "piano".into());

        update_keyboard_hints(&mut hints, statuses);

        // // all updated values should be correct
        assert_eq!(*hints.get(&'p').unwrap(), LetterState::Correct);
        assert_eq!(*hints.get(&'i').unwrap(), LetterState::Correct);
        assert_eq!(*hints.get(&'a').unwrap(), LetterState::NotPresent);
        assert_eq!(*hints.get(&'n').unwrap(), LetterState::NotPresent);
        assert_eq!(*hints.get(&'o').unwrap(), LetterState::Incorrect);

        // existing values should not be present
        assert_eq!(hints.get(&'x'), None);

        // check new guess word where already correct letter is now in incorrect position; hint
        // should show 'correct' because it was correct once
        let statuses: Vec<LetterStatus> = check(wordle.clone(), "smile".into());

        // update hints again
        update_keyboard_hints(&mut hints, statuses);

        // i => already correct, it should still be correct
        assert_eq!(*hints.get(&'i').unwrap(), LetterState::Correct);
    }
}
