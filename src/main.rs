extern crate docopt;
#[macro_use]
extern crate serde_derive;

use docopt::Docopt;
use std::thread::{park_timeout, spawn};
use std::time::Duration;

const USAGE: &'static str = "
Domodoro. A pomodoro utility that can be used as a D-Bus service.

Usage: domodoro [options] <task-name>

Options:
  -h --help                 Show this screen.
  -v --version              Show version.

  -l --length=<minutes>     Length of your pomodoro [default: 5]
  -b --break=<minutes>      Length of your breaks [default: 2]
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_task_name: String,
    flag_length: u64,
    flag_break: u64,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let len = Duration::from_secs(args.flag_length);
    let brk = Duration::from_secs(args.flag_break);

    let mut pomodoro = Pomodoro::new(len, brk);
    pomodoro.start();

    let handle = spawn(move || {
        for state in pomodoro {
            println!("Got {:?}", state);
        }
    });
    handle.join().unwrap()
}

#[derive(Debug, Clone, Copy)]
enum State {
    Stop,
    Pause,
    Work,
    LongBreak,
    ShortBreak,
}

struct Pomodoro {
    dur: Duration,
    brk: Duration,
    prev_state: State,
    state: State,
}

const MAX_PAUSE_TIME: u64 = 3_600;

impl Pomodoro {
    fn new(dur: Duration, brk: Duration) -> Pomodoro {
        Pomodoro {
            dur,
            brk,
            prev_state: State::Pause,
            state: State::Pause,
        }
    }

    fn start(&mut self) {
        self.change(State::Work);
    }

    fn change(&mut self, state: State) {
        self.prev_state = self.state;
        self.state = state;
        let dur = match state {
            State::Work => self.dur,
            State::ShortBreak => self.brk,
            State::LongBreak => self.brk * 4,
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
            State::Work => State::ShortBreak,
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
