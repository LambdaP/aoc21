use std::str;
use anyhow::Result;

// pub fn main() {
//     if let Ok(numbers) = parse_lines() {
//         let n = ninc(&numbers);
//         println!("{}", n);
//         let n = ninc3(&numbers);
//         println!("{}", n);
//     } else {
//         eprintln!("Error");
//     }
// }

fn parse_lines(lines: &[&[u8]]) -> Result<Vec<usize>> {
    let mut numbers: Vec<usize> = Vec::new();

    for l in lines {
        numbers.push(str::from_utf8(l)?.parse::<usize>()?);
    }

    Ok(numbers)
}

pub fn part1(lines: &[&[u8]]) -> Result<usize> {
    let numbers = parse_lines(lines)?;
    let mut count: usize = 0;

    if let Some((x, xs)) = numbers.split_first() {
        let mut x = x;
        for n in xs {
            if n > x { count += 1; }
            x = n;
        }
    }

    Ok(count)
}

pub fn part2(lines: &[&[u8]]) -> Result<usize> {
    let numbers = parse_lines(lines)?;
    let mut count: usize = 0;

    if let Some((x,xs)) = numbers.split_first() {
        let mut a = x;
        if let Some((x,xs)) = xs.split_first() {
            let mut b = x;
            if let Some((x,xs)) = xs.split_first() {
                let mut c = x;
                for n in xs {
                    if a < n {
                        count += 1;
                    }
                    a = b;
                    b = c;
                    c = n;
                }
            }
        }
    }
    Ok(count)
}

// 
// #[cfg(test)]
// mod tests {
//     use super::*;
// 
//     #[test]
//     fn ninc_empty() {
//         assert_eq!(ninc(&[]), 0);
//     }
// 
//     #[test]
//     fn ninc_noinc() {
//         assert_eq!(ninc(&[1,0]), 0);
//         assert_eq!(ninc(&[10,1,0]), 0);
//         assert_eq!(ninc(&[100,10,1,0]), 0);
//     }
// 
//     #[test]
//     fn ninc_inc() {
//         assert_eq!(ninc(&[10,0,100,0,1000]), 2);
//         assert_eq!(ninc(&[0,1,2,3,4,5]), 5);
//     }
// }
