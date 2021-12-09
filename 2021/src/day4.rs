use std::str;
use anyhow::Result;

type Bingo = [[u8;5];5];

fn parse_grid<'a,'b>(lines: &'a[&'a[u8]]) -> Result<Bingo> {
    let grid = lines.iter().map(|l| parse_grid_row(l));

    let mut res = [[0;5];5];
    for (i,row) in grid.enumerate() {
        res[i] = row?;
    }

    Ok(res)
}

fn parse_grid_row<'a,'b>(row: &'a[u8]) -> Result<[u8;5]> {
    let numbers = str::from_utf8(row)?
        .split_ascii_whitespace()
        .map(|x| u8::from_str_radix(x.trim(), 10));

    let mut res = [0;5];
    for (i,s) in numbers.enumerate() {
        res[i] = s?;
    }

    Ok(res)
}

fn parse_first_line(l: &[u8]) -> Result<Vec<u8>> {
    let numbers = str::from_utf8(l)?
        .split_terminator(',')
        .map(|x| u8::from_str_radix(x.trim(), 10));

    let mut res = Vec::new();
    for x in numbers {
        res.push(x?);
    }

    Ok(res)
}

fn play_bingo(grid: Bingo, draws: &[u8]) -> Option<(usize, usize)>{
    let mut row_tot = [5;5];
    let mut col_tot = [5;5];
    let mut mgrid = grid.clone();

    let mut ix = 0;
    for &d in draws {
        for i in 0..5 {
            for j in 0..5 {
                if grid[i][j] == d {
                    row_tot[i] -= 1;
                    col_tot[j] -= 1;
                    mgrid[i][j] = 0;
                    if row_tot[i] == 0 || col_tot[j] == 0 {
                        let mut sum: usize = 0;
                        for x in mgrid.concat() {
                            sum += usize::from(x);
                        }
                        return Some((ix, usize::from(d) * sum));
                    }
                }
            }
        }
        ix += 1;
    }

    None
}

pub fn part1(lines: &[&[u8]]) -> Result<usize> {
    let draws = parse_first_line(lines[0])?;
    let grids = lines[2..]
        .split(|l| l.len() == 0)
        .map(parse_grid);

    let mut games = Vec::new();
    for g in grids {
        if let Some(x) = play_bingo(g?, &draws) {
            games.push(x);
        }
    }

    games.sort_by_key(|k| k.0);
    games.first()
        .ok_or_else(|| anyhow::anyhow!("no game finished"))
        .map(|x| x.1)

}

pub fn part2(lines: &[&[u8]]) -> Result<usize> {
    let draws = parse_first_line(lines[0])?;
    let grids = lines[2..]
        .split(|l| l.len() == 0)
        .map(parse_grid);

    let mut games = Vec::new();
    for g in grids {
        if let Some(x) = play_bingo(g?, &draws) {
            games.push(x);
        }
    }

    games.sort_by_key(|k| k.0);
    games.last()
        .ok_or_else(|| anyhow::anyhow!("no game finished"))
        .map(|x| x.1)
}
