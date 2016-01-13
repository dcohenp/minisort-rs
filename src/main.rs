use std::vec;
// use std::io;

enum SortDirection {
    Forward,
    Reverse,
}

struct SortArgs {
    direction: SortDirection,
    inputs: Vec<Box<std::io::Read>>,
}

impl SortArgs {
    fn parse(args: &mut Iterator<Item=String>) -> SortArgs {
    //fn parse(args: &mut std::env::ArgsOs) -> SortArgs {

        // Skip arg 0 which is program name
        let progname = args.next().unwrap();

        println!("progname={}", progname);

        let mut direction = SortDirection::Forward;

        let inputs = match args.next() {
            // Single-arg means stdin
            None => vec![Box::new(std::io::stdin()) as Box<std::io::Read>],
            Some(argname) => panic!("Files not implemented"),
        };

        SortArgs {
            direction: direction,
            inputs: inputs,
        }
    }
}

fn main() {

    let sort_args = SortArgs::parse(&mut std::env::args());

}
