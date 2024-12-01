use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let stdin = io::stdin();
    let parsed = stdin.lock().lines().map(|a| { let a = a.expect("where's my line");
                                                let mut a = a.split_whitespace();
                                                let s1 = a.next().expect("where's my word");
                                                let s2 = a.next().expect("where's my word");
                                                (u64::from_str(s1).expect("where's my int"),
                                                 u64::from_str(s2).expect("where's my int"))});
    let (mut left, mut right): (Vec<_>, Vec<_>) = parsed.unzip();
    left.sort();
    right.sort();
    
    let result: i64 = left.iter().zip(right.iter()).map(|(&a,&b)| (b as i64 - a as i64).abs()).sum();
    println!("{}", result);
}
