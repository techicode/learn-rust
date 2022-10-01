// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function
// * Create and store at least 3 people in a vector

// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
struct Person {
    age: i32,
    name: String,
    favorite_color: String,
}

fn main() {
    // * Create and store at least 3 people in a vector
    let persons = vec![
        Person {
            age: 21,
            name: "Louis".to_owned(),
            favorite_color: "Green".to_owned(),
        },
        Person {
            age: 15,
            name: "Pepe".to_owned(),
            favorite_color: "White".to_owned(),
        },
        Person {
            age: 32,
            name: "Jack".to_owned(),
            favorite_color: "Black".to_owned(),
        },
    ];

    // * Iterate through the vector using a for..in loop
    // * Use an if expression to determine which person's info should be printed
    for person in persons {
        if person.age > 18 {
            println!("----");
            println!("Name: {:?}", person.name);
            println!("Age: {:?}", person.age);
            println!("Favorite Color: {:?}", person.favorite_color);
        }
    }
}
