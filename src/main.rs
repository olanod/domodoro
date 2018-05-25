
extern crate docopt;
#[macro_use]
extern crate serde_derive;

use docopt::Docopt;
use std::time::Duration;
use std::thread::sleep;

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

    println!("Start working on {}", args.arg_task_name);
    loop {
        sleep(len);
        println!("worked for {}s, now break!", len.as_secs());
        sleep(brk);
        println!("rested for {}s, now continue!", brk.as_secs());
    }
}
