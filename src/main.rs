extern crate docopt;
#[macro_use]
extern crate serde_derive;

use docopt::Docopt;
use std::sync::mpsc;
use std::thread::{spawn};
use std::time::Duration;

const USAGE: &'static str = "
Domodoro. A pomodoro utility that can be used as a D-Bus service.

Usage: domodoro [options] <task-name>

Options:
  -h --help                 Show this screen.
  -v --version              Show version.

  -l --length=<minutes>     Length of your pomodoro [default: 20]
  -b --break=<minutes>      Length of your breaks [default: 5]
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_task_name: String,
    flag_length: u16,
    flag_break: u16,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let len = Duration::from_secs((args.flag_length).into());
    let brk = Duration::from_secs((args.flag_break).into());

    let (tx, rx) = mpsc::channel();

    let mut pomodoro = Pomodoro::new(len, brk);
    pomodoro.start();

    spawn(move || {
        // work a bit
        tx.send(pomodoro.next());
        tx.send(pomodoro.next());
        tx.send(pomodoro.next());
        tx.send(pomodoro.next());
    });

    for state in rx {
        println!("Got {:?}", state);
    }
}

#[derive(Debug, Clone, Copy)]
enum State {
    Stop,
    Pause,
    Work,
    //LongBreak,
    ShortBreak,
}

struct Pomodoro {
    dur: Duration,
    short_brk: Duration,
    long_brk: Duration,
    prev_state: State,
    state: State,
}

impl Pomodoro {
    fn new(dur: Duration, brk: Duration) -> Pomodoro {
        Pomodoro {
            dur,
            short_brk: brk,
            long_brk: brk * 4,
            prev_state: State::Stop,
            state: State::Pause,
        }
    }

    fn start(&mut self) {
        self.change(State::Work);
    }

    fn change(&mut self, new: State) -> State {
        self.prev_state = self.state;
        self.state = new;
        println!("Changed from {:?} to {:?}", self.prev_state, self.state);
        self.state
    }
}

impl Iterator for Pomodoro {
    type Item = State;

    fn next(&mut self) -> Option<State> {
        let next = match self.state {
            State::Work => State::ShortBreak,
            State::ShortBreak => State::Work,
            //State::LongBreak => State::Work,
            State::Pause => self.prev_state,
            State::Stop => State::Stop,
        };
        match next { State::Stop => None, _ => Some(self.change(next)) } 
    }
}
