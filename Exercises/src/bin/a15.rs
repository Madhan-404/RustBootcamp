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

fn main() {

    let ticket = vec![
        Tickets::Backstage("Madhan".to_string(), 20),
        Tickets::Vip("Raj".to_string(), 35),
        Tickets::Standard(10)

    ];

    for ticket in ticket {
        
        match ticket {
            Tickets::Backstage(name,price) => println!("{} {}", name, price),
            Tickets::Vip(name,price ) => println!("{} {}",name, price),
            Tickets::Standard(price) => println!("{}",price)
        }
    }

}

enum Tickets {
    Backstage(String,i32),
    Vip(String,i32),
    Standard(i32)
}

