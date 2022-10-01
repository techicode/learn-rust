// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:

// * Use an enum for the box color
enum ColorBox {
    Red,
    Gray,
    Black,
}

impl ColorBox {
    fn print(&self) {
        match self {
            ColorBox::Red => println!("Color: Red"),
            ColorBox::Gray => println!("Color: Gray"),
            ColorBox::Black => println!("Color: Black"),
        }
    }
}

struct Dimension {
    height: f64,
    width: f64,
    depth: f64,
}

impl Dimension {
    fn print(&self) {
        println!("Height: {:?}", self.height);
        println!("width: {:?}", self.width);
        println!("Depth: {:?}", self.depth);
    }
}

// * Use a struct to encapsulate the box characteristics
struct Box {
    color: ColorBox,
    dimension: Dimension,
    weight: f64,
}

impl Box {
    // * Implement functionality on the box struct to create a new box
    fn new(weight: f64, color: ColorBox, dimension: Dimension) -> Self {
        Self {
            color,
            dimension,
            weight,
        }
    }

    // Create a  box with default values
    fn create_default_box() -> Self {
        Self {
            color: ColorBox::Black,
            dimension: Dimension {
                height: 65.4,
                width: 44.3,
                depth: 10.2,
            },
            weight: 20.0,
        }
    }

    // * Implement functionality on the box struct to print the characteristics
    fn print_box(&self) {
        println!("--Box Information--");
        self.color.print();
        self.dimension.print();
        println!("Weight: {:?}", self.weight);
    }
}

fn main() {
    let new_default_box = Box::create_default_box();
    new_default_box.print_box();

    let small_dimension = Dimension {
        width: 10.2,
        height: 5.0,
        depth: 2.5,
    };

    let small_box = Box::new(4.2, ColorBox::Gray, small_dimension);
    small_box.print_box();
}
