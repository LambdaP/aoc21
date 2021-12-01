use std::io;
use std::error;

fn main() {
    if let Ok(numbers) = parse_lines() {
        let n = ninc(&numbers);
        println!("{}", n);
        let n = ninc3(&numbers);
        println!("{}", n);
    } else {
        eprintln!("Error");
    }
}

fn parse_lines() -> Result<Vec<usize>, Box<dyn error::Error>> {
    let mut buf = String::new();
    let mut numbers: Vec<usize> = Vec::new();

    while io::stdin().read_line(&mut buf)? > 0 {
        let n = buf.trim().parse::<usize>()?;
        numbers.push(n);
        buf.clear();
    }

    Ok(numbers)
}

fn ninc(sl: &[usize]) -> usize {
    let mut count: usize = 0;

    if let Some((x, xs)) = sl.split_first() {
        let mut x = x;
        for n in xs {
            if n > x { count += 1; }
            x = n;
        }
    }

    count
}

fn ninc3(sl: &[usize]) -> usize {
    let mut count: usize = 0;

    if let Some((x,xs)) = sl.split_first() {
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
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ninc_empty() {
        assert_eq!(ninc(&[]), 0);
    }

    #[test]
    fn ninc_noinc() {
        assert_eq!(ninc(&[1,0]), 0);
        assert_eq!(ninc(&[10,1,0]), 0);
        assert_eq!(ninc(&[100,10,1,0]), 0);
    }

    #[test]
    fn ninc_inc() {
        assert_eq!(ninc(&[10,0,100,0,1000]), 2);
        assert_eq!(ninc(&[0,1,2,3,4,5]), 5);
    }
}
