fn main() {
    let x = add(1, 2);

    println!("{:?}", x);

    let num = 0;

    if num > 0 {
        println!("Positive number!")
    } else if num < 0 {
        println!("Negative number!")
    } else {
        println!("It's a zero!")
    }

    let mut i = 0;
    loop {
        if i > 5 {
            break;
        }
        println!("{:?}", i);
        i = i + 1;
    }

    while i > 0 {
        println!("{:?} value", i);
        i = i - 1;
    }


    test();
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn test() {
    fn first_name() {
        println!("Luis")
    }
    fn last_name() {
        println!("Sáez")
    }

    first_name();
    last_name();
}