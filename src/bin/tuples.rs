fn main() {
    let tup = ("patata", "potato", "papa");

    println!("{:?}", tup.0);
    println!("{:?}", tup.1);
    println!("{:?}", tup.2);

    fn one_two_three() -> (i32, i32, i32) {
        (4, 10, 24)
    }

    let (x, y, z) = one_two_three();
    println!("x: {:?}", x);
    println!("y: {:?}", y);
    println!("z: {:?}", z);

    enum Access {
        Full,
    }

    let (employe, access) = ("Pepito", Access::Full);
}