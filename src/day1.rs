use std::io::{self, BufRead};
use std::str::FromStr;
use std::collections::HashSet;
fn main() {
    let stdin = io::stdin();
    let parsed = stdin.lock().lines().map(|a| { let a = a.expect("where's my line");
                                                let mut a = a.split_whitespace();
                                                let s1 = a.next().expect("where's my word");
                                                let s2 = a.next().expect("where's my word");
                                                (u32::from_str(s1).expect("where's my int"),
                                                 u32::from_str(s2).expect("where's my int"))});
    
    /* day 1 */
    let (mut left, mut right): (Vec<_>, Vec<_>) = parsed.unzip();
    left.sort();
    right.sort();
    
    let result1: i32 = left.iter().zip(right.iter()).map(|(&a,&b)| (b as i32 - a as i32).abs()).sum();
    println!("{}", result1);

    /* day 2 */
    let left_set: HashSet<&u32> = left.iter().collect();
    let result2: u32 = right.into_iter().filter(|a| left_set.contains(a)).sum();
    println!("{}", result2);
}
