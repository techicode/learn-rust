struct Locker {
    name: String,
    assign: Option<i32>,
}

fn main() {
    let locker_1 = Locker {
        name: "Jack".to_owned(),
        assign: Some(1234),
    };
    let locker_2 = Locker {
        name: "Steffan".to_owned(),
        assign: None,
    };

    match locker_1.assign {
        Some(assign) => println!("The assignment number is {:?}", assign),
        None => println!("There's no assignment number for this locker"),
    }

    match locker_2.assign {
        Some(assign) => println!("The assignment number is {:?}", assign),
        None => println!("There's no assignment number for this locker"),
    }
}
