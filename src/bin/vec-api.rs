// example for vec (from the api docs)

fn main() {
    let numbers = vec![1, 4, 50];

    match numbers.is_empty() {
        true => println!("The vector is empty!"),
        false => println!("The vector is not empty!"),
    }
}