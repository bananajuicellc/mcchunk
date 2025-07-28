
mod calculator;
mod cli;

fn main() {
    cli::print_chunk_corners(calculator::Coord { x: -17, y: 2, z: -37 })
}
