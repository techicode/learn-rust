#[derive(Debug, Copy, Clone)]
enum Position {
    Manager,
    Supervisor,
    Worker,
}

#[derive(Debug, Copy, Clone)]
struct Employee {
    position: Position,
    work_hours: i32,
}

fn print_employee(emp: Employee) {
    println!("{:?}", emp);
}

fn main() {
    let pepito = Employee {
        position: Position::Worker,
        work_hours: 50,
    };

    print_employee(pepito);
    print_employee(pepito);
}