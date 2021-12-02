use std::io;
use std::io::Read;
use std::error;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
enum Heading {
    U(usize),
    D(usize),
    F(usize),
}

impl FromStr for Heading {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sp = s.split_whitespace();
        let s0 = sp.next().expect("empty string");
        let x = sp .next()
                    //.ok_or(...)
                    .expect("no field separator")
                    .parse::<usize>()?;

        return match s0 as &str {
            "up"      => Ok(Heading::U(x)),
            "down"    => Ok(Heading::D(x)),
            "forward" => Ok(Heading::F(x)),
            _         => panic!() // Err(Box::new())
        }
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let headings = input
                    .lines()
                    .filter_map(|l| l.parse::<Heading>().ok());

    let mut x: usize = 0;
    let mut z: usize = 0;

    for h in headings.clone() {
        match h {
            Heading::U(n) => { z -= n }
            Heading::D(n) => { z += n }
            Heading::F(n) => { x += n }
        }
    }

    println!("{}", x*z);

    let mut x: usize = 0;
    let mut z: usize = 0;
    let mut aim: usize = 0;

    for h in headings {
        match h {
            Heading::U(n) => { aim -= n }
            Heading::D(n) => { aim += n }
            Heading::F(n) => { x += n; z += aim * n }
        }
    }

    println!("{}", x*z);

    Ok(())
}
