use std::io::{self, Read};
use std::collections::HashSet;
fn walk(grid : &[Vec<char>], y: usize, x: usize) -> HashSet<(isize,isize)> {
    let mut y = y as isize;
    let mut x = x as isize;
    let mut dir: (isize, isize) = (0,-1);
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    while y < grid.len() as isize && x < grid[0].len() as isize && y > 0 && x > 0 {
        visited.insert((x,y));
        if grid.get((y+dir.1) as usize).and_then(|a| a.get((x+dir.0) as usize)) == Some(&'#') {
            dir = (-dir.1, dir.0);
        }
        //println!("{}, {}", x, y);
        x += dir.0;
        y += dir.1;
    }
    visited
}

fn is_obstruction(grid : &[Vec<char>], y: usize, x: usize, yo: isize, xo: isize) -> bool {
    let mut y = y as isize;
    let mut x = x as isize;
    let mut dir: (isize, isize) = (0,-1);
    let mut visited: HashSet<(isize, isize, isize, isize)> = HashSet::new();
    while y < grid.len() as isize && x < grid[0].len() as isize && y > 0 && x > 0 {
        if visited.contains(&(x,y,dir.0,dir.1)) {
            return true
        }
        visited.insert((x,y, dir.0, dir.1));
        while ((y+dir.1) == yo && (x+dir.0) == xo) || grid.get((y+dir.1) as usize).and_then(|a| a.get((x+dir.0) as usize)) == Some(&'#') {
            dir = (-dir.1, dir.0);
        }
        //println!("{}, {}", x, y);
        x += dir.0; 
        y += dir.1;
    }
    false
}

fn compute(parsed: &[Vec<char>]) -> (usize, usize) {
    let y: usize = parsed.iter().position(|r| r.contains(&'^')).unwrap();
    let x: usize = parsed[y].iter().position(|&r| r == '^').unwrap();
    let path = walk(parsed, y, x);
    let mut res2 = 0;
    let res1 = path.len();
    for (xo,yo) in path {
        let yo = yo as usize;
        let xo = xo as usize;
        if is_obstruction(parsed, y, x, yo as isize, xo as isize) && (y != yo || x != xo) {
            //println!("x = {}, y = {}", xo, yo);
            res2 += 1;
        }
    }
    (res1, res2)
}
fn main() {
    let mut input = String::new();

    let _ = io::stdin().read_to_string(&mut input);
    use std::time::Instant;
    let parsed: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let now = Instant::now();
    let (r1, r2) = compute(&parsed);
    let elapsed = now.elapsed();
    println!("{} {}", r1, r2);

    println!("Elapsed: {:.2?}", elapsed);
}
