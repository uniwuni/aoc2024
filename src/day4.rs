use std::io::{self, Read};

fn valid_point(grid: &[Vec<char>], x: isize, y: isize) -> bool {
    !(y < 0 || y >= grid.len() as isize || x < 0 || x >= grid[y as usize].len() as isize)
}

fn get_neighbour_dirs(
    grid: &[Vec<char>],
    x: usize,
    y: usize,
    direction: Option<(isize, isize)>,
) -> Vec<(isize, isize)> {
    if let Some((dx, dy)) = direction {
        if valid_point(grid, x as isize + dx, y as isize + dy) {
            return vec![(dx, dy)];
        } else {
            return vec![];
        }
    }
    //println!("wah");
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .iter()
    .filter(|&(dx, dy)| valid_point(grid, x as isize + dx, y as isize + dy))
    .copied()
    .collect()
}

fn count_words_at(
    grid: &[Vec<char>],
    x1: usize,
    y1: usize,
    string: &str,
    direction: Option<(isize, isize)>,
) -> usize {
    //println!("{} {} {}", string, x1, y1);
    if string.is_empty() {
        return 1;
    }
    get_neighbour_dirs(grid, x1, y1, direction)
        .iter()
        .filter(|&(dx, dy)| {
            string.starts_with(grid[(*dy + y1 as isize) as usize][(*dx + x1 as isize) as usize])
        })
        .map(|(dx, dy)| {
            count_words_at(
                grid,
                (*dx + x1 as isize) as usize,
                (*dy + y1 as isize) as usize,
                &string[1..],
                Option::Some((*dx, *dy)),
            )
        })
        .sum()
}
fn is_mas_x(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    if grid[y][x] != 'A'
        || !(valid_point(grid, x as isize - 1, y as isize - 1)
            && valid_point(grid, x as isize + 1, y as isize + 1))
    {
        false
    } else {
        ((grid[y - 1][x - 1] == 'M' && grid[y + 1][x + 1] == 'S')
            || (grid[y - 1][x - 1] == 'S' && grid[y + 1][x + 1] == 'M'))
            && ((grid[y + 1][x - 1] == 'M' && grid[y - 1][x + 1] == 'S')
                || (grid[y + 1][x - 1] == 'S' && grid[y - 1][x + 1] == 'M'))
    }
}

fn compute(parsed: &[Vec<char>]) -> (usize, usize) {
    let mut res1: usize = 0;
    let mut res2: usize = 0;
    for y in 0..parsed.len() {
        for x in 0..parsed[y].len() {
            if parsed[y][x] == 'X' {
                res1 += count_words_at(parsed, x, y, "MAS", None);
            }
            if is_mas_x(&parsed, x, y) {
                res2 += 1;
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        // avoid parsing issues
        let input = String::from(
            "....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX",
        )
        .lines()
        .map(|x| x.chars().collect())
        .collect();

        assert_eq!(compute(input).0, 18);
    }
    #[test]
    fn example2() {
        // avoid parsing issues
        let input: Vec<Vec<char>> = String::from(
            ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........",
        )
        .lines()
        .map(|x| x.chars().collect())
        .collect();
        assert_eq!(compute(input).1, 9);
    }

    #[test]
    fn example3() {
        // avoid parsing issues
        let input: Vec<Vec<char>> = String::from("..X...\n.SAMX.\n.A..A.\nXMAS.S\n.X....")
            .lines()
            .map(|x| x.chars().collect())
            .collect();
        assert_eq!(compute(input).0, 4);
    }
}
