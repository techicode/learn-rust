enum Discount {
    Percent(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: i32,
}

fn main() {
    let n = 5;
    match n {
        3 => println!("Three!"),
        other => println!("number: {:?}", other)
    };

    let flat = Discount::Flat(5);
    match flat {
        Discount::Flat(2) => println!("Just 2!"),
        Discount::Flat(amount) => println!("Flat discount of {:?}", amount),
        _ => (),
    }

    let concert = Ticket {
        event: "Concert".to_owned(),
        price: 55,
    };

    match concert {
        Ticket { price: 50, ..} => println!("Just $50!"),
        Ticket { price, ..} => println!("Price is ${:?}", price),
    }
}