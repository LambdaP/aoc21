use std::str;
use anyhow::Result;
use anyhow::*; // anyhow!(), bail!()

type Wind = ((usize,usize),(usize,usize));

fn parse_wind(line: &[u8]) -> Result<Wind> {
    let str = str::from_utf8(line)?;

    let mut w = ((0,0),(0,0));

    let (s,str) = str
        .split_once(' ')
        .ok_or_else(|| anyhow!("parse error"))?;

    let (x,y) = s
        .split_once(',')
        .ok_or_else(|| anyhow!("parse error"))?;

    w.0.0 = usize::from_str_radix(x,10)?;
    w.0.1 = usize::from_str_radix(y,10)?;

    let (_,s) = str
        .split_once(' ')
        .ok_or_else(|| anyhow!("parse error"))?;

    let (x,y) = s
        .split_once(',')
        .ok_or_else(|| anyhow!("parse error"))?;

    w.1.0 = usize::from_str_radix(x,10)?;
    w.1.1 = usize::from_str_radix(y,10)?;

    Ok(w)
}

pub fn part1(lines: &[&[u8]]) -> Result<usize> {
    let mut winds = Vec::new();
    for w in lines.iter().map(|l| parse_wind(l)) {
        let w = w?;
        if w.0.0 == w.1.0 || w.0.1 == w.1.1 {
            winds.push(w);
        }
    }

    let mut points: Vec<(usize,usize)> = Vec::new();

    for w in winds {
        if w.0.0 == w.1.0 {
            let (a,b) = if w.0.1 < w.1.1 { (w.0.1,w.1.1) } else { (w.1.1,w.0.1) };
            for j in a..b+1 {
                points.push((w.0.0, j));
            }
        } else if w.0.1 == w.1.1 {
            let (a,b) = if w.0.0 < w.1.0 { (w.0.0,w.1.0) } else { (w.1.0,w.0.0) };
            for i in a..b+1 {
                points.push((i, w.0.1));
            }
        }
    }

    points.sort();

    Ok(group(&points).iter()
        .filter(|x| x.len() > 1)
        .count())
}

fn group<T>(xx: &[T]) -> Vec<&[T]> where T: PartialEq {
    let mut res = Vec::new();

    if xx.len() == 0 {
        return res;
    }

    let mut i0 = 0;

    for (i,x) in xx[0..].iter().enumerate() {
        if *x != xx[i0] {
            res.push(&xx[i0..i]);
            i0 = i;
        }
    }

    res
}

pub fn part2(lines: &[&[u8]]) -> Result<usize> {
    let mut winds = Vec::new();
    for w in lines.iter().map(|l| parse_wind(l)) {
        winds.push(w?);
    }

    let mut points: Vec<(usize,usize)> = Vec::new();

    for w in winds {
        let (p0,p1) = if w.0.0 <= w.1.0 { (w.0,w.1) } else { (w.1,w.0) };
        if p0.0 == p1.0 {
            let (a,b) = if p0.1 < p1.1 { (p0.1,p1.1) } else { (p1.1,p0.1) };
            for j in a..b+1 {
                points.push((p0.0, j));
            }
        } else if p0.1 == p1.1 {
            for i in p0.0..p1.0+1 {
                points.push((i, p0.1));
            }
        } else {
            if p0.1 <= p1.1 { // ascending
                for i in 0..(p1.0-p0.0+1) {
                    points.push((p0.0+i,p0.1+i));
                }
            } else { // descending
                for i in 0..(p1.0-p0.0+1) {
                    points.push((p0.0+i,p0.1-i));
                }
            }
        }
    }

    points.sort();

    Ok(group(&points).iter()
        .filter(|x| x.len() > 1)
        .count())
}
