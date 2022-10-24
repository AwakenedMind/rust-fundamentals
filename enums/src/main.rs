enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let go = Direction::Up;
    let dir = which_way(go);

    println!("Direction: {:?}", dir)
}

// enums
// 1. can only be one variant at any given time
// 2. allows for more robust software when paired with "match"
// 3. easier to read
fn which_way(go: Direction) -> &'static str {
    match go {
        Direction::Up => return "up",
        Direction::Down => return "down",
        Direction::Left => return "left",
        Direction::Right => return "right",
    };
}
