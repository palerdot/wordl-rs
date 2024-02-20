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
                let wordle_indices = get_all_letter_indices(guess_letter, wordle.clone());
                let guess_indices = get_all_letter_indices(guess_letter, guess.clone());

                let coloring_chances = wordle_indices.len();
                let future_start = guess_indices.partition_point(|&i| i == position);
                let future_occurences = guess_indices[future_start..].iter();
                let future_chances = future_occurences.len();

                // eg: e: ennui [0] <-- where [2,4]
                // eg: e: drove [4] <-- evoke [0,4]
                //
                // should we leave coloring this time?
                let should_leave = future_chances >= coloring_chances;
                if !should_leave {
                    let is_incorrect = !wordle_indices.contains(&position);
                    if is_incorrect {
                        output.push(LetterStatus {
                            letter: guess_letter,
                            status: LetterState::Incorrect,
                        });
                    } else {
                        output.push(LetterStatus {
                            letter: guess_letter,
                            status: LetterState::NotPresent,
                        });
                    }
                } else {
                    // we are leaving this letter out as there are matches in the future (word
                    // occurence)
                    output.push(LetterStatus {
                        letter: guess_letter,
                        status: LetterState::NotPresent,
                    });
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
                LetterState::Incorrect => {
                    // we will update only if it is not already correct
                    if let Some(current_status) = hints.get(&status.letter) {
                        if *current_status != LetterState::Correct {
                            hints.insert(status.letter.clone(), status.status.clone());
                        }
                    }
                }
                LetterState::Unknown | LetterState::NotPresent => {
                    // we will update only if it not in Correct/Incorrect stage
                    if let Some(current_status) = hints.get(&status.letter) {
                        if *current_status != LetterState::Correct
                            && *current_status != LetterState::Incorrect
                        {
                            hints.insert(status.letter.clone(), status.status.clone());
                        }
                    }
                }
                // correct is always correct!
                LetterState::Correct => {
                    hints.insert(status.letter.clone(), LetterState::Correct);
                }
            }
        }
    }

    hints
}

fn get_all_letter_indices(letter: char, word: String) -> Vec<usize> {
    let mut output: Vec<usize> = Vec::new();

    word.chars()
        .enumerate()
        .into_iter()
        .for_each(|(index, word_letter)| {
            if word_letter == letter {
                output.push(index);
            }
        });

    output
}

#[cfg(test)]
mod tests {
    use crate::wordle::model::{LetterState, LetterStatus};
    use crate::wordle::utils::*;
    use std::collections::HashMap;

    #[test]
    fn test_letter_indices() {
        let output = get_all_letter_indices('e', "ennui".into());
        assert_eq!(output, vec![0]);

        let output = get_all_letter_indices('e', "where".into());
        assert_eq!(output, vec![2, 4]);

        let output = get_all_letter_indices('e', "drove".into());
        assert_eq!(output, vec![4]);

        let output = get_all_letter_indices('e', "evoke".into());
        assert_eq!(output, vec![0, 4]);
    }

    #[test]
    fn test_status_random() {
        let output = check("ennui".into(), "where".into());

        let expected: Vec<LetterStatus> = vec![
            LetterStatus {
                letter: 'w',
                status: LetterState::NotPresent,
            },
            LetterStatus {
                letter: 'h',
                status: LetterState::NotPresent,
            },
            LetterStatus {
                letter: 'e',
                status: LetterState::NotPresent,
            },
            LetterStatus {
                letter: 'r',
                status: LetterState::NotPresent,
            },
            LetterStatus {
                letter: 'e',
                status: LetterState::Incorrect,
            },
        ];

        assert_eq!(output, expected);

        // test another
        let output = check("drove".into(), "evoke".into());

        let expected: Vec<LetterStatus> = vec![
            LetterStatus {
                letter: 'e',
                status: LetterState::NotPresent,
            },
            LetterStatus {
                letter: 'v',
                status: LetterState::Incorrect,
            },
            LetterStatus {
                letter: 'o',
                status: LetterState::Correct,
            },
            LetterStatus {
                letter: 'k',
                status: LetterState::NotPresent,
            },
            LetterStatus {
                letter: 'e',
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

        // NEW WORDLE
        // RESET everything
        let wordle = "below";
        hints.clear();

        // check new wordle
        // WORDLE - below; Word - Hello
        let statuses: Vec<LetterStatus> = check(wordle.into(), "hello".into());

        // update hints again
        update_keyboard_hints(&mut hints, statuses);

        // i => already correct, it should still be correct
        assert_eq!(*hints.get(&'l').unwrap(), LetterState::Correct);
    }
}
