fn main() {
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    fn cardinal_point(go: Direction) -> &'static str {
        match go {
            Direction::Down => "South",
            Direction::Up => "North",
            Direction::Left => "West",
            Direction::Right => "East",
        }
    }

    println!("{:?}", cardinal_point(Direction::Up));
}