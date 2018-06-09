mod domodoro;

extern crate docopt;
#[macro_use]
extern crate serde_derive;

use docopt::Docopt;
use domodoro::Pomodoro;
use std::thread::spawn;
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
