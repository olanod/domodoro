use std::time::Duration;
use std::thread::park_timeout;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    Stop,
    Pause,
    Work,
    LongBreak,
    ShortBreak,
}

pub struct Pomodoro {
    dur: Duration,
    brk: Duration,
    pomodoros: i16,
    prev_state: State,
    state: State,
}

const MAX_PAUSE_TIME: u64 = 3_600;
const LONG_BREAK_AFTER: i16 = 4;
const LONG_BREAK_RATIO: u32 = 4;

impl Pomodoro {
    pub fn new(dur: Duration, brk: Duration) -> Pomodoro {
        Pomodoro {
            dur,
            brk,
            pomodoros: 0,
            prev_state: State::Pause,
            state: State::Pause,
        }
    }

    pub fn start(&mut self) {
        self.change(State::Work);
    }

    fn change(&mut self, state: State) {
        self.prev_state = self.state;
        self.state = state;
        if state == State::Work {
            self.pomodoros += 1;
            println!("pomodoros -> {}", self.pomodoros);
        }
        let dur = match state {
            State::Work => self.dur,
            State::ShortBreak => self.brk,
            State::LongBreak => self.brk * LONG_BREAK_RATIO,
            State::Pause => Duration::from_secs(MAX_PAUSE_TIME),
            State::Stop => Duration::from_secs(0),
        };
        println!(
            "Changed from {:?} to {:?}, wating {:?}",
            self.prev_state, self.state, dur
        );
        park_timeout(dur);
    }
}

impl Iterator for Pomodoro {
    type Item = State;

    fn next(&mut self) -> Option<State> {
        let next = match self.state {
            State::Work => if self.pomodoros % LONG_BREAK_AFTER != 0 {
                State::ShortBreak
            } else {
                State::LongBreak
            },
            State::ShortBreak => State::Work,
            State::LongBreak => State::Work,
            State::Pause => self.prev_state,
            State::Stop => State::Stop,
        };
        match next {
            State::Stop => None,
            _ => {
                self.change(next);
                Some(next)
            }
        }
    }
}
