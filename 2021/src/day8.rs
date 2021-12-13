use anyhow::*; // Result<T>, anyhow!(), bail!()

type Digit = u8;

fn w2b(w: &[u8]) -> Result<Digit> {
    let mut res = 0;

    for &x in w {
        res += match x {
            b'a' => 1 << 0,
            b'b' => 1 << 1,
            b'c' => 1 << 2,
            b'd' => 1 << 3,
            b'e' => 1 << 4,
            b'f' => 1 << 5,
            b'g' => 1 << 6,
            _    => bail!("word parse error")
        };
    }

    Ok(res)
}

fn count_ones(mut d: Digit) -> u8 {
    let mut res = 0;

    while d > 0 {
        if d % 2 == 1 {
            res += 1;
        }
        d /= 2;
    }

    res
}

fn decode_signals(scrambled: &[Digit;10]) -> [Digit;10] {
    let mut res = [0;10];

    let mut scrambled = scrambled.clone();
    scrambled.sort_by_cached_key(|&x| count_ones(x));

    res[1] = scrambled[0];
    res[7] = scrambled[1];
    res[4] = scrambled[2];

    for &d in &scrambled[3..6] {
        if count_ones(d ^ res[1]) == 3 {
            res[3] = d;
        } else if count_ones(d ^ res[4]) == 3 {
            res[5] = d;
        } else {
            res[2] = d;
        }
    }

    for &d in &scrambled[6..9] {
        if count_ones(d ^ res[1]) == 6 {
            res[6] = d;
        } else if count_ones(d ^ res[4]) == 2 {
            res[9] = d;
        } else {
            res[0] = d;
        }
    }

    res[8] = scrambled[9];

    res
}

fn parse_line(line: &[u8]) -> Result<[Digit;14]> {
    let mut res: [Digit;14] = [0;14];

    for (i, x) in (0..10).zip(line[0..59].split(|&x| x == b' ')) {
        res[i] = w2b(x)?;
    }

    for (i, x) in (10..).zip(line[61..].split(|&x| x == b' ')) {
        res[i] = w2b(x)?;
    }

    Ok(res)
}

pub fn part1(lines: &[&[u8]]) -> Result<usize> {
    let mut res = 0;
    for l in lines {
        let digits = parse_line(l)?;
        for &d in &digits[10..] {
            res += match count_ones(d) {
                2|3|4|7 => 1,
                _ => 0
            };
        }
    }

    Ok(res)
}

pub fn part2(lines: &[&[u8]]) -> Result<usize> {
    let mut res = 0;
    for l in lines {
        let digits = parse_line(l)?;
        let signals: &[Digit;10] = unsafe {
            std::mem::transmute(&digits[0])
        };
        let decoded = decode_signals(&signals);

        let mut output = 0;
        for &x in &digits[10..] {
            output *= 10;
            for (j,&d) in decoded.iter().enumerate() {
                if x == d { output += j; }
            }
        }

        res += output;
    }

    Ok(res)
}
