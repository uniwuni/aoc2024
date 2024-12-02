use std::io::{self, BufRead};
use std::str::FromStr;

fn is_safe(seq: &[u32]) -> bool {
    let diffs = seq.windows(2).map(|a| (a[1] as i32) - (a[0] as i32));
    let mut sign: i32 = 0;
    for d in diffs {
        if sign != 0 && d.signum() != sign {
            return false;
        }
        if d < -3 || d == 0 || d > 3 {
            return false;
        }
        sign = d.signum();
    }
    true
}

fn is_weakly_safe(vec: Vec<u32>) -> bool {
    (0..vec.len()).any(|n| {
        let seq: Vec<u32> = vec
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != n)
            .map(|(_, &v)| v)
            .collect();
        is_safe(&seq)
    })
}

fn compute<A>(parsed: A) -> (u32, u32)
where
    A: Iterator<Item = Vec<u32>>,
{
    /* day 1 */
    let (result1, result2) = parsed.fold((0, 0), |(acc1, acc2), x| {
        if is_safe(&x) {
            (acc1 + 1, acc2 + 1)
        } else if is_weakly_safe(x) {
            (acc1, acc2 + 1)
        } else {
            (acc1, acc2)
        }
    });
    (result1, result2)
}
fn main() {
    let stdin = io::stdin();
    let parsed = stdin.lock().lines().map(|a| {
        let a = a.expect("where's my line");
        a.split_whitespace()
            .map(|x| u32::from_str(x).expect("no int"))
            .collect::<Vec<_>>()
    });
    use std::time::Instant;
    let now = Instant::now();

    let (r1, r2) = compute(parsed);
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
        let data = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        assert_eq!(compute(data.into_iter()), (2, 4));
    }
}
