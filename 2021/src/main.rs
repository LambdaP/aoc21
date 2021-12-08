use std::io;
use std::io::Read;
use std::env;
use std::string::String;
use anyhow::Result;

mod day1;
mod day2;
mod day3;

type Solver = fn(&[&[u8]]) -> anyhow::Result<usize>;

fn main() -> Result<()> {
    let day = env::args().nth(1).expect("no args");

    let (part1, part2) : (Solver, Solver) = match &day as &str {
        // "--day1" => (day1::day1, 0),
        "--day2" => (day2::part1, day2::part2),
        "--day3" => (day3::part1, day3::part2),
        _      => { eprintln!("error: {} is not a valid option", day); panic!() }
    };

    let lines = stdin_bytes()?;
    let lines: Vec<&[u8]> = lines
            .split(|x| *x == b'\n')
            .collect();

    let res1 = part1(&lines)?;
    println!("{}", res1);

    let res2 = part2(&lines)?;

    println!("{}", res2);

    // 0b011001100111

    // println!("{} * {} = {}", 0b011001100111, 0b101010001, 0b011001100111 * 0b101010001);

    Ok(())
}

fn stdin_bytes() -> Result<Vec<u8>> {
    let mut res: Vec<u8> = Vec::new();
    io::stdin().read_to_end(&mut res)?;
    Ok(res)
}

fn stdin_utf8() -> Result<String> {
    let mut res: String = String::new();
    io::stdin().read_to_string(&mut res)?;
    Ok(res)
}
