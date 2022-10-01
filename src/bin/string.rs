fn print_it(data: &str) {
    println!("{:?}", data);
}

struct Employee {
    name: String,
}

fn main() {
    println!("A string slice &str");
    let owned_string = "owned string".to_owned();
    let another_owned = String::from("Another");

    print_it(&owned_string);
    print_it(&another_owned);

    let emp_name = "Jack".to_owned();
    let emp = Employee {
        name: emp_name,
    };

    print_it(&emp.name);
}