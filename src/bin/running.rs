fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn print_result(result: i32) {
    println!("{:?}", result);
}

fn main() {
    let sum1 = sum(1, 5);
    print_result(sum1);
}