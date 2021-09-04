#![feature(iter_zip)]
use std::io::{self, Read};
use std::str::FromStr;
use std::result::Result;
use std::num::ParseIntError;
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut input = input.split("\n\n");

    let rules = input.next().unwrap();

    let my_ticket = input.next().unwrap();
    let my_ticket = my_ticket.split("\n").nth(1).unwrap();

    let nearby_tickets = input.next().unwrap();
    let nearby_tickets: Result<Vec<_>, Box<dyn std::error::Error>> = nearby_tickets.split("\n").skip(1).filter(|&s| !s.is_empty()).map(Ticket::from_str).collect();
    let nearby_tickets = nearby_tickets?;


    let my_ticket = Ticket::from_str(my_ticket)?;

    let rules = Rule::from_str(rules)?;

    dbg!(&my_ticket);
    dbg!(&rules);
    dbg!(&nearby_tickets);

    let sum: usize = nearby_tickets.iter().map(|t| t.invalid_numbers_rate(&rules)).sum();
    
    println!("{}", sum);

    Ok(())
}

#[derive(Debug)]
struct Rule {
    rules: Vec<Field>,
    valid_numbers: HashSet<usize>,
}

#[derive(Debug, Clone)]
struct Field {
 name: String,
 range1: (usize, usize),
 range2: (usize, usize),
}

impl FromStr for Field {
 type Err = Box<dyn std::error::Error>;
 fn from_str(s: &str) -> Result<Self, Self::Err> {
    // example: 
    // departure location: 30-260 or 284-950
    let mut s = s.split(": ");
    let name = s.next().ok_or("Couldn't parse name")?;
    let name = name.to_owned();

    let nums = s.next().ok_or("No numbers to parse in the form '1-2 or 3-4'")?;

    let mut nums = nums.split(" or ");

    let range1 = nums.next().ok_or("Failed to parse first range")?;
    let range2 = nums.next().ok_or("Failed to parse second range")?;

    let range1: Result<Vec<usize>, ParseIntError> = range1.split("-")
        .map(str::parse).collect();
    let range1 = range1?;
    let range1 = (range1[0], range1[1]);


    let range2: Result<Vec<usize>, ParseIntError> = range2.split("-")
        .map(str::parse).collect();
    let range2 = range2?;
    let range2 = (range2[0], range2[1]);

    Ok(Self{name, range1, range2})
 }

}

impl Rule {
    fn validate_ticket(&self, ticket: &Ticket) -> usize {
       std::iter::zip(self.rules.clone(), ticket.ticket.clone()).map(|(f, t)| {
            let range1 = f.range1.0..=f.range1.1;
            let range2 = f.range2.0..=f.range2.1;

            if range1.contains(&t) || range2.contains(&t) {
                0
            } else {
                dbg!(&t);
                t
            }


       }).sum()
    }

    fn invalid_numbers_rate(&self, ticket: &Ticket) -> usize {
        ticket.ticket.iter().filter(|num| !self.valid_numbers.contains(num)).sum()
    }

}

impl FromStr for Rule {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        
        let rules: Result<Vec<Field>, Box<dyn std::error::Error>> = s.split("\n").map(Field::from_str).collect();
        let rules = rules?;

        let valid_numbers: HashSet<usize> = rules.iter().map( |f| {
            let range1 = f.range1.0..=f.range1.1;
            let range1: HashSet<usize> = range1.collect();
            
            let range2 = f.range2.0..=f.range2.1;
            let range2: HashSet<usize> = range2.collect();

            range1.union(&range2).cloned().collect()

        }).reduce(|acc: HashSet<usize>, element| acc.union(&element).cloned().collect()).ok_or("Could collect valid numbers")?;

        dbg!(&valid_numbers);

        Ok(Self {rules, valid_numbers})
    }


}

#[derive(Debug)]
struct Ticket {
ticket: Vec<usize>,
}

impl Ticket {
    // oops this is not what the question was asking for
fn is_valid(&self, rule: &Rule) -> usize{
    // 0 if valid, integer with rate  if not 
    rule.validate_ticket(self)
}

fn invalid_numbers_rate(&self, rule: &Rule) -> usize{
    rule.invalid_numbers_rate(self)
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
