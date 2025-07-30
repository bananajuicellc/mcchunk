
pub struct Coord {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

pub const fn calculate_chunk_corners(pos: &Coord) -> [Coord; 4] {
    let north = pos.z - pos.z.rem_euclid(16);
    let west = pos.x - pos.x.rem_euclid(16);
    let south = north + 15;
    let east = west + 15;

    [
        Coord {
            x: west,
            y: pos.y,
            z: north,
        },
        Coord {
            x: east,
            y: pos.y,
            z: north,
        },
        Coord {
            x: west,
            y: pos.y,
            z: south,
        },
        Coord {
            x: east,
            y: pos.y,
            z: south,
        },
    ]
}
