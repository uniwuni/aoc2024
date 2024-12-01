use std::io::{self, BufRead};
use std::str::FromStr;
use std::collections::HashMap;

fn compute<A>(parsed: A) -> (u32, u32)
  where A: Iterator<Item = (u32, u32)> {
    /* day 1 */
    let (mut left, mut right): (Vec<_>, Vec<_>) = parsed.unzip();
    left.sort();
    right.sort();
    
    let result1: u32 = left.iter().zip(right.iter()).map(|(&a,&b)| a.abs_diff(b)).sum();


    /* day 2 */
    let left_map: HashMap<&u32,u32> = left.iter().fold(HashMap::new(), |mut acc, n|
        {*acc.entry(n).or_insert(0) += 1;
        acc});
    let result2: u32 = right.into_iter().map(|n| n * left_map.get(&n).unwrap_or(&0)).sum();
    (result1, result2)
}
fn main() {
    let stdin = io::stdin();
    let parsed = stdin.lock().lines().map(|a| { let a = a.expect("where's my line");
                                                let mut a = a.split_whitespace();
                                                let s1 = a.next().expect("where's my word");
                                                let s2 = a.next().expect("where's my word");
                                                (u32::from_str(s1).expect("where's my int"),
                                                 u32::from_str(s2).expect("where's my int"))});
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
        let data = vec![(3,4),(4,3),(2,5),(1,3),(3,9),(3,3)];
        assert_eq!(compute(data.into_iter()), (11,31));
    }
}
