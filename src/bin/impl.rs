struct Temperature {
    degrees_c: f64,
}

impl Temperature {

    fn freezing() -> Self {
        Self {
            degrees_c: 32.0,
        }
    }

    fn show_temp(&self) {
        println!("The temp in fahrenheit is: {:?}", self.degrees_c);
    }
}

fn main() {
    let hot = Temperature {
        degrees_c: 98.6,
    };

    hot.show_temp();

    let cold = Temperature::freezing();
    cold.show_temp();

}