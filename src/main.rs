use std::env;

use tudu::run;

fn main() {
    let args: Vec<String> = env::args().collect();

    run(args);
}
