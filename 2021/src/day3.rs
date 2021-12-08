use std::vec::Vec;
use std::iter::*;
use anyhow::Result;
use anyhow::*;

pub fn part1(lines: &[&[u8]]) -> Result<usize> {

    let mut cc: Vec<isize> = vec![0; lines[0].len()];

    for l in lines {
        for (i, b) in l.iter().enumerate() {
            match b {
                b'0' => cc[i] -= 1,
                b'1' => cc[i] += 1,
                _    => panic!()
            }
        }
    }

    let (mut gam, mut eps) = (0,0);

    for i in cc {
        gam *= 2;
        eps *= 2;
        if i > 0 {
            gam += 1;
        } else {
            eps += 1;
        }
    }

    Ok(gam * eps)
}

#[derive(Debug)]
struct BTree {
    zero: (usize, Option<Box<BTree>>),
    one:  (usize, Option<Box<BTree>>)
}

impl BTree {
    fn new() -> Self {
        BTree { zero: (0,None), one: (0,None) }
    }

    fn count_line(&mut self, line: &[u8]) {
        let mut p: &mut BTree = self;

        for b in line {
            match b {
                b'0' => {
                    p.zero.0 += 1;
                    p = p.zero.1
                        .get_or_insert_with(|| Box::new(Self::new()));
                },
                b'1' => {
                    p.one.0 += 1;
                    p = p.one.1
                        .get_or_insert_with(|| Box::new(Self::new()));
                },
                _ => panic!()
            }
        }
    }
}

pub fn part2(lines: &[&[u8]]) -> Result<usize> {
    let mut t = BTree::new();

    for l in lines {
        t.count_line(l);
    }

    let oxy = oxy(&t);
    let co2 = co2(&t);

    Ok(oxy * co2)
}

fn oxy(t: &BTree) -> usize {
    let mut res = 0;
    let mut p: &BTree = &t;

    while p.zero.0 > 0 || p.one.0 > 0 {
        res *= 2;
        if p.zero.0 <= p.one.0 {
            res += 1;
            if let Some(q) = &p.one.1 {
                p = &q;
            }
        } else if let Some(q) = &p.zero.1 {
            p = &q;
        }
    }

    res
}

fn co2(t: &BTree) -> usize {
    let mut res = 0;
    let mut t: &BTree = &t;
    while t.zero.0 > 0 || t.one.0 > 0 {
        res *= 2;
        if t.zero.0 == 0 || (0 < t.one.0 && t.one.0 < t.zero.0) {
            res += 1;
            if let Some(q) = &t.one.1 {
                t = &q;
            }
        } else if let Some(q) = &t.zero.1 {
            t = &q;
        }
    }

    res
}
