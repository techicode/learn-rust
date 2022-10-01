#[derive(Debug)]
enum Mouse {
    LeftClick,
    RightClick,
    MiddleClick,
    Scroll(i32),
    Move(i32, i32),
}

enum PromoDiscount {
    NewUser,
    Holiday(String),
}

enum Discount {
    Percent(f64),
    Flat(i32),
    Promo(PromoDiscount),
    Custom(String),
}

fn main() {
    let scroll_dir = Mouse::Scroll(-2);
    let mouse_pos = Mouse::Move(12, 34);

    println!("{:?}", scroll_dir); // Scroll(-2)
    println!("{:?}", mouse_pos); // Move(12, 34)

    let christmas_promo = PromoDiscount::Holiday("Christmas".to_owned());
    let final_discount = Discount::Promo(christmas_promo);
}
