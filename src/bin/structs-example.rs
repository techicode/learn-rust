// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

fn main() {

    // define a different types of drinks
    enum DrinkFlavors {
        Margarita,
        Mojito,
        Cosmopolitan,
        Daiquiri,
    }

    // make an struct of a flavor with enum and the fluid_once a decimal f64 double
    struct StoreDrink {
        flavor: DrinkFlavors,
        fluid_ounce: f64,
    }

    // this function print the info of any object type StoreDrink
    fn print_drink(drink: StoreDrink) {
        match drink.flavor {
            DrinkFlavors::Cosmopolitan => println!("The drink flavor is Cosmopolitan!"),
            DrinkFlavors::Mojito => println!("The drink flavor is Mojito!"),
            DrinkFlavors::Margarita => println!("The drink flavor is Margarita!"),
            DrinkFlavors::Daiquiri => println!("The drink flavor is Daiquiri!"),
        }

        println!("The ounce of the drink is {:?}", drink.fluid_ounce);
    }

    // create an object of StoreDrink
    let mojito = StoreDrink {
        flavor: DrinkFlavors::Mojito,
        fluid_ounce: 24.5,
    };

    // call the print function with the object mojito
    print_drink(mojito);
}
