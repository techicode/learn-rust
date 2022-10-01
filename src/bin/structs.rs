fn main() {
    struct ShippingBox {
        depth: i32,
        width: i32,
        height: i32,
    }

    let my_box = ShippingBox {
        depth: 3,
        width: 2,
        height: 5,
    };

    let tall = my_box.height;

    println!("The box is {:?} units tall", tall);
}