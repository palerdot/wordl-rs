use rand::seq::SliceRandom;
use std::collections::HashMap;

use crate::wordle;

pub type KeyboardHints = HashMap<char, LetterState>;

#[derive(Debug, Default)]
pub struct Model {
    // main wordle word
    pub wordle: String,

    // data
    pub valid_wordles: Vec<String>,
    pub valid_guesses: Vec<String>,

    // user guesses
    pub active_guess: String,
    pub guesses: Vec<Vec<LetterStatus>>,

    pub running_state: RunningState,
    pub keyboard_hints: KeyboardHints,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GameResult {
    CorrectGuess,
    WrongGuess,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Waiting,
    Calculating,
    Over(GameResult),
    Done,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Message {
    Listen(char),
    Erase,
    CalculateStart,
    AnimateGuess(usize, Vec<LetterStatus>),
    CalculateEnd(Vec<LetterStatus>),
    Reset,
    Quit,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LetterStatus {
    pub letter: char,
    pub status: LetterState,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum LetterState {
    #[default]
    Unknown,
    NotPresent,
    Correct,
    Incorrect,
}

impl Model {
    pub fn new() -> Self {
        let valid_guesses = wordle::data::valid_guesses();
        let valid_wordles = wordle::data::valid_wordles();
        let wordle = valid_wordles
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string();

        let default_model = Model::default();

        Model {
            wordle,
            valid_guesses,
            valid_wordles,
            ..default_model
        }
    }

    pub fn reset(&mut self) {
        self.wordle = self
            .valid_wordles
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string();
        self.active_guess = "".into();
        self.guesses.clear();
        self.keyboard_hints.clear();
        self.running_state = RunningState::Waiting;
    }
}
