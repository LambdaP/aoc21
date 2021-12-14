use anyhow::*; // Result<T>, anyhow!(), bail!()

type Point = (usize, usize);

fn neighbors(p: Point) -> Vec<Point> {
    let mut ns = vec![(p.0+1,p.1), (p.0,p.1+1)];
    if p.0 > 0 {
        ns.push((p.0-1,p.1));
    }
    if p.1 > 0 {
        ns.push((p.0,p.1-1));
    }
    ns
}

fn sinks(grid: &[&[u8]]) -> Vec<Point> {
    let mut res = Vec::new();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] <
                neighbors((i,j)).iter() //ns.iter()
                .filter_map(
                    |&(x,y)| grid.get(x)
                        .and_then(|l| l.get(y))
                        .and_then(|&b| Some(b))
                ).min()
                .unwrap_or(b':') {
                res.push((i,j));
            }
        }
    }

    res
}

pub fn part1(lines: &[&[u8]]) -> Result<usize> {
    let mut res: usize = 0;

    for (i,j) in sinks(lines) {
        res += 1 + (lines[i][j] - b'0') as usize;
    }

    Ok(res)
}

fn count_basin(grid: &mut[&mut[u8]], p: Point) -> usize {
    let mut res = 0;
    let mut stack: Vec<Point> = vec![p];

    while let Some((i,j)) = stack.pop() {
        if let Some(&b) = grid.get(i).and_then(|r| r.get(j)) {
            if b < b'9' {
                grid[i][j] = b':';
                res += 1;
                stack.append(&mut neighbors((i,j)));
            }
        }
    }

    res
}

pub fn part2(lines: &[&[u8]]) -> Result<usize> {
    let mut grid: Vec<Vec<u8>> = lines.into_iter().map(|l| l.to_vec()).collect();
    let mut grid: Vec<&mut [u8]> = grid.iter_mut().map(|r| r.as_mut_slice()).collect();

    let mut basins: Vec<usize> = sinks(lines)
        .into_iter()
        .map(|p| count_basin(grid.as_mut_slice(), p))
        .collect();
    basins.sort();

    let mut res = 1;
    for _ in 0..3 {
        res *= basins.pop().ok_or_else(|| anyhow!("not enough basins"))?;
    }
    Ok(res)
}
