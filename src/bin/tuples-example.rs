fn main() {
    fn get_cartesian_coord () -> (i32, i32) {
        (24, 4)
    }

    let (x, y) = get_cartesian_coord();

    if y > 5 {
        println!("The Y coord is greater than 5")
    } else if y < 5 {
        println!("The Y coord is less than 5")
    } else {
        println!("The Y coord is equal to 5")
    }
}