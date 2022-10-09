struct Customer {
    age: Option<i32>,
    email: String,
}

struct GroceryItem {
    name: String,
    qty: i32,
}

fn find_quantity(name: &str) -> Option<i32> {
    let groceries = vec![
        GroceryItem { name: "Banana".to_owned(), qty: 10},
        GroceryItem { name: "Egg".to_owned(), qty: 1},
        GroceryItem { name: "Bread".to_owned(), qty: 5},
    ];

    for grocery in groceries {
        if grocery.name == name {
            return Some(grocery.qty)
        }
    }
    None
}

fn main() {
    let mark = Customer {
        age: Some(22), email: "emilio@gmail.com".to_owned(),
    };
    let becky = Customer {
        age: None, email: "beckysiana@live.cl".to_owned(),
    };

    match mark.age {
        Some(age) => println!("Mark's age is {:?}", age),
        None => println!("Mark doesn't have age xD"),
    }

    match becky.age {
        Some(age) => println!("Becky's age is {:?}", age),
        None => println!("Becky doesn't have age xD"),
    }

    println!("{:?}", find_quantity("Egg")); // > Some(1)
    println!("{:?}", find_quantity("Banana")); // > Some(10)
    println!("{:?}", find_quantity("Pizza")); // > None
}