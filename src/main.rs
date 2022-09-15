/* Prompt
    "CPS 142 Assignment # 2 Due:
    Create an abstract class that encapsulates a movie ticket. The movie ticket has one data member: theater
    number. Include an abstract method that returns the ticket price, this will need 2 parameters, one to indicate if
    ticket is for an adult or a child and the second parameter to indicate if it’s a regular or reserved seat.
    Create two subclasses for the movie ticket class:
    Matinee movie ticket: ticket prices are $6.00 for adults and children with reserved seat an extra $1.00
    NEED DATA MEMEBERS HERE!
    Evening movie ticket: ticket prices are $8.50 for adults and $6.00 for children with reserved seat and
    extra $2.00
    Include the usual constructors, accessors, mutators, toString, copy or copy constructor and equals methods in all
    of the classes, except where not allowed. The abstract class should throw an IllegalArgumentException if there
    is an attempt to set the theater number to less than 1 or greater than 8.
    Write a separate program then interacts with users so that movie tickets may be purchased. Have user enter:
    theater number, matinee or evening, child or adult, and regular or reserved seat. Create an instance of the
    appropriate class and store all tickets purchased in a single ArrayList that only accepts abstract movie tickets
    and subclasses. When a ticket is purchased the price is displayed. This program should contain a loop to allow
    the purchase of tickets until stopped by the user.
    Before the program terminates, the list of all items in the ArrayList is displayed, including the total amount
    collected for all tickets.
    I will let you decide how the user enters information, but no GUI for this one, we may do that later.
    Submit printed copies of all commented and formatted code; and the output from program. The execution
    should demonstrate
    Do not use GUI for the I/O.
*/

// External Module Includes
    use text_io::*;
//

// Local Module Includes
    mod ticket;
//

// Global Type Definitions
//

// Functions
fn main() -> () {
    //Ask user for information deciding the type of 
    //ticket that is to be dispensed to them.

    let mut tickets = vec![0];
    
    let mut ad: i32 = 0;
    let mut ni: i32 = 0;
    let mut re: i32 = 0;

    let mut selection = 0;
    
    while selection == 0 {
        print!("Are you an adult? (1/0)");
        ad = read!();
        print!("Is it a night time showing? (1/0)");
        ni = read!();
        print!("Do you want your seat reserved? (1/0)");
        re = read!();

        tickets.push(ticket::process_ticket(ad, ni, re));
        
        println!("Exit? (1/0)");
        selection = read!();
    }

    print!("{}", tickets.len());
    
    for x in 0..tickets.len() {
        println!("{}", ticket::read_ticket_data(tickets[x]));
    }
}
//