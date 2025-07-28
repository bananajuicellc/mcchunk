use super::calculator::Coord;
use super::calculator::calculate_chunk_corners;

pub fn print_chunk_corners(pos: Coord) {
    let corners = calculate_chunk_corners(pos);
    println!("Chunk contains coordinates (x, y, z):");
    for corner in corners {
        println!("\t{}\t{}\t{}", corner.x, corner.y, corner.z); 
    }
    println!("and each block in-between. The y coordinates can be ignored.");
}