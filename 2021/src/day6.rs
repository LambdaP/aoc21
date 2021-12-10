use std::str;
use anyhow::Result;
use anyhow::*; // anyhow!(), bail!()

fn parse_fishes(line: &[u8]) -> Result<[usize;9]> {
    let mut fishes = [0;9];
    for bytes in line.split(|&x| x == b',') {
        match usize::from_str_radix(str::from_utf8(bytes)?,10)? {
            i if i <= 8 => fishes[i] += 1,
            _ => bail!("parsed number is greater than 8")
        }
    }

    Ok(fishes)
}

fn day_tick(mut fishes: [usize;9]) -> [usize;9] {
    let ripe = fishes[0];

    for i in 0..8 {
        fishes[i] = fishes[i+1];
    }

    fishes[6] += ripe;
    fishes[8] = ripe;

    fishes
}

pub fn part1(lines: &[&[u8]]) -> Result<usize> {
    let mut fishes = parse_fishes(lines.iter().next().ok_or_else(|| anyhow!("empty file"))?)?;
    for _ in 0..80 {
        fishes = day_tick(fishes);
    }

    let nfishes = fishes.iter().sum();
    Ok(nfishes)
}

pub fn part2(lines: &[&[u8]]) -> Result<usize> {
    let mut fishes = parse_fishes(lines.iter().next().ok_or_else(|| anyhow!("empty file"))?)?;
    for _ in 0..256 {
        fishes = day_tick(fishes);
    }

    let nfishes = fishes.iter().sum();
    Ok(nfishes)
}
