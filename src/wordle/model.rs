#[derive(Debug, Default)]
pub struct Model {
    pub wordle: String,
    pub active_guess: String,
    pub guesses: Vec<Vec<LetterStatus>>,
    pub running_state: RunningState,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Waiting,
    Calculating,
    Over,
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
