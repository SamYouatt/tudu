use tudu::parse_command;

fn main() {
    println!("Hello, world!");

    let args = vec!["tudu"];

    parse_command(args).expect("oops");
}
