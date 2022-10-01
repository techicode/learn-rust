fn main() {
    enum Direction {
        Left,
        Right,
    }

    let go = Direction::Right;
    match go {
        Direction::Right => println!("Go to the right!"),
        Direction::Left => println!("Go to the right!"),
    }
}