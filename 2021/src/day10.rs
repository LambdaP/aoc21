use anyhow::*; // Result<T>, anyhow!(), bail!()

enum DyckParseError {
    Corrupted(u8),
    Incomplete(Vec<u8>)
}


fn dyck_validate(s: &[u8]) -> Option<DyckParseError> {
    use DyckParseError::*;
    let mut stack = vec![];

    for &x in s {
        match x {
            b'('|b'['|b'{'|b'<' => stack.push(x),
            b')'|b']'|b'}'|b'>' => {
                if let Some(y) = stack.pop() {
                    match (y,x) {
                        (b'(',b')')
                            |(b'[',b']')
                            |(b'{',b'}')
                            |(b'<',b'>') => { },
                        _ => {
                            return Some(Corrupted(x))
                        }
                    }
                } else {
                    return Some(Corrupted(x))
                }
            },
            _ => panic!()
        }
    }
    if stack.len() > 0 {
        Some(Incomplete(stack))
    } else {
        None
    }
}

fn score_completion(mut s: Vec<u8>) -> Result<usize> {
    let mut res = 0;
    while let Some(b) = s.pop() {
        res *= 5;
        res += match b {
            b'(' => 1,
            b'[' => 2,
            b'{' => 3,
            b'<' => 4,
            _    => bail!("invalid input")
        }
    }

    Ok(res)
}

pub fn part1(lines: &[&[u8]]) -> Result<usize> {
    use DyckParseError::*;
    let mut res = 0;
    for l in lines {
        if let Some(Corrupted(b)) = dyck_validate(l) {
            res += match b {
                b')' => 3,
                b']' => 57,
                b'}' => 1197,
                b'>' => 25137,
                _ => bail!("impossible")
            }
        }
    }

    Ok(res)
}

pub fn part2(lines: &[&[u8]]) -> Result<usize> {
    use DyckParseError::*;
    let mut scores = Vec::new();
    for l in lines {
        if let Some(Incomplete(s)) = dyck_validate(l) {
            scores.push(score_completion(s)?);
        }
    }

    scores.sort();
    Ok(scores[scores.len()/2])
}
