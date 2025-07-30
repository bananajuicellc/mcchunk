use std::process;

mod calculator;
mod cli;


fn print_usage_and_exit(command: String, error: String) -> ! {
    eprintln!("{}\n", error);
    eprintln!("Usage:");
    eprintln!("\t{} x y z", command);
    process::exit(-1);
}


fn parse_args(mut args: std::env::Args) -> calculator::Coord {
    let command = match args.next() {
        Some(value) => value,
        None => "mcchunk".to_string(),
    };

    let x: i32 = match args.next() {
        Some(value ) => match value.parse() {
            Ok(x) => x,
            Err(err) => print_usage_and_exit(command, format!("Expected x to be an integer, but got {}:\n{}", value, err)),
        },
        None => print_usage_and_exit(command, "Expected x, but missing".to_string()),
    };

    let y: i32 = match args.next() {
        Some(value ) => match value.parse() {
            Ok(y) => y,
            Err(err) => print_usage_and_exit(command, format!("Expected y to be an integer, but got {}:\n{}", value, err)),
        },
        None => print_usage_and_exit(command, "Expected y, but missing".to_string()),
    };

    let z: i32 = match args.next() {
        Some(value ) => match value.parse() {
            Ok(z) => z,
            Err(err) => print_usage_and_exit(command, format!("Expected z to be an integer, but got {}:\n{}", value, err)),
        },
        None => print_usage_and_exit(command, "Expected z, but missing".to_string()),
    };

    calculator::Coord { x: x, y: y, z: z }
}

fn main() {
    let args = std::env::args();
    let pos = parse_args(args);
    cli::print_chunk_corners(&pos);
}
