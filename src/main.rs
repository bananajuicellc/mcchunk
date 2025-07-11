#[derive(Debug, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

const fn calculate_chunk_corners(pos: Coord) -> (Coord, Coord, Coord, Coord) {
    let chunk_start = Coord {
        x: pos.x - pos.x.rem_euclid(16),
        y: pos.y,
        z: pos.z - pos.z.rem_euclid(16),
    };
    let chunk_end = Coord {
        x: chunk_start.x + 15,
        y: pos.y,
        z: chunk_start.z + 15,
    };
    (
        chunk_start,
        Coord {
            x: chunk_start.x,
            y: pos.y,
            z: chunk_end.z,
        },
        Coord {
            x: chunk_end.x,
            y: pos.y,
            z: chunk_start.z,
        },
        chunk_end,
    )
}

fn main() {
    println!(
        "Hello {:?}!",
        calculate_chunk_corners(Coord { x: -17, y: 2, z: -37 })
    );
}
