// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Ticket {
    Backstage(i32, String),
    Vip(i32, String),
    Standard(i32),
}

fn main() {
    let all_tickets = vec![
        Ticket::Backstage(69, "Pepe".to_owned()),
        Ticket::Vip(120, "Jack".to_owned()),
        Ticket::Standard(39),
    ];

    for ticket in all_tickets {
        match ticket {
            Ticket::Backstage(price, owner) => {
                println!("The price is ${:?} and the owner is {:?}", price, owner)
            }
            Ticket::Vip(price, owner) => {
                println!("The price is ${:?} and the owner is {:?}", price, owner)
            }
            Ticket::Standard(price) => println!("The price is ${:?}", price),
        }
    }
}
