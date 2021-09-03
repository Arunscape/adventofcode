use std::io::{self, Read};
use std::str::FromStr;
use std::result::Result;
use std::num::ParseIntError;

fn main() -> io::Result<()>{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut input = input.split("\n\n");

    let rules = input.next().unwrap();

    let my_ticket = input.next().unwrap();
    let my_ticket = my_ticket.split("\n").nth(1).unwrap();

    let nearby_tickets = input.next().unwrap();
    let nearby_tickets: Vec<_> = nearby_tickets.split("\n").skip(1).filter(|&s| !s.is_empty()).collect();


    dbg!(&rules, &my_ticket, &nearby_tickets);

    let my_ticket = Ticket::from_str(my_ticket);

    dbg!(my_ticket);

    Ok(())
}


struct Rule {
}

impl Rule {

}

impl FromStr for Rule {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
todo!()
    }

}

#[derive(Debug)]
struct Ticket {
ticket: Vec<usize>,
}

impl Ticket {
fn is_valid(&self, rule: Rule) {
todo!()
}
}
impl FromStr for Ticket {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ticket: Result<Vec<usize>, ParseIntError> = s.split(",").map(str::parse).collect();
        let ticket = ticket?;

        Ok(Self {ticket})
    }

}
