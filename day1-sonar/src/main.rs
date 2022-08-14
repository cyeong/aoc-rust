use std::{env, process};

use day1_sonar::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arg: {err}");
        process::exit(1);
    });

    /*
    We use if let rather than unwrap_or_else to check whether run returns an Err value and call
    process::exit(1) if it does. The run function doesn’t return a value that we want to unwrap in the same
    way that Config::build returns the Config instance.
    Because run returns () in the success case, we only care about detecting an error,
    so we don’t need unwrap_or_else to return the unwrapped value, which would only be ().

    The bodies of the if let and the unwrap_or_else functions are the same in both cases:
    we print the error and exit.
    */
    if let Err(e) = day1_sonar::run(config) {
        println!("Application error: {e}");
        process::exit(1)
    }
}
