fn main() {
    let my_num = 4;
    let is_over_5 = if my_num > 5 {
        true
    } else {
        false
    };

    let is_lower_5 = my_num < 5;


    let message = match my_num {
        1 => "Hello",
        2 => "Yo",
        3 => "wadup",
        4 => "Ohayou!",
        _ => "Bye"
    };

    println!("{:?}", message);

    enum Menu {
        Burger,
        Fries,
        Drink,
    }

    let item = Menu::Drink;
    let drink_type = "Water";

    let order_placed = match item {
        Menu::Drink => drink_type == "water",
        _ => true,
    };
}