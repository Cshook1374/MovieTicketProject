/* Tickets are a 3 bit binary number. A ticket is initialized to zero, and 
has the following offsets added to it: 1 if the ticket is for an adult, 2 
if the ticket is a nighttime showing, and 4 if the ticket is a reserved 
seat.
*/

pub fn process_ticket(adult: i32, night: i32, reserved: i32) -> i32 {
    let mut ticket: i32 = 0;
    
    if adult == 1 {
        ticket += 1; 
    } 
    if night == 1 {
        ticket += 2;
    }
    if reserved == 1 {
        ticket += 4;
    }
    return ticket;
}

pub fn read_ticket_data(ticket: i32) -> f32 {
    match ticket {
        0 => return 6.00,
        1 => return 6.00,
        2 => return 6.00,
        3 => return 8.50,
        4 => return 7.00,
        5 => return 7.00,
        6 => return 7.00,
        7 => return 9.50,
        _ => return 0.00
    }
}
