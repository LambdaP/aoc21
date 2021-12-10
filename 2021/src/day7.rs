use std::str;
use anyhow::Result;
use anyhow::*; // anyhow!(), bail!()

fn parse_positions(line: &[u8]) -> Result<Vec<usize>> {
    let mut res = Vec::new();
    for bytes in line.split(|&x| x == b',') {
        res.push(usize::from_str_radix(str::from_utf8(bytes)?,10)?);
    }

    Ok(res)
}

pub fn part1(lines: &[&[u8]]) -> Result<usize> {
    let mut positions = parse_positions(lines[0])?;
    positions.sort();
    let med: usize = positions[positions.len()/2];
    let mut res = 0;
    for x in positions {
        res += if x < med { med - x } else { x - med };
    }

    Ok(res)
}

pub fn part2(lines: &[&[u8]]) -> Result<usize> {
    let positions: Vec<usize> = parse_positions(lines[0])?;
    let sum: usize = positions.iter().sum();
    let avg = (sum as f64)/(positions.len() as f64);

    let mut res_lo: usize = 0;
    let mut res_hi: usize = 0;
    let avg_lo = f64::floor(avg) as usize;
    let avg_hi = f64::ceil(avg) as usize;
    for x in positions {
        let abs = if x <= avg_lo { avg_lo - x } else { x - avg_lo };
        res_lo += abs * (abs+1);
        let abs = if x <= avg_hi { avg_hi - x } else { x - avg_hi };
        res_hi += abs * (abs+1);
    }

    let res = if res_lo < res_hi { res_lo } else { res_hi }/2;

    Ok(res)
}
