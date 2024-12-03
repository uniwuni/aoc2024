use regex::Regex;
use std::io::{self, Read};

fn compute(parsed: String) -> (u32, u32) {
    let re2 = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut on = true;
    let mut res1 = 0;
    let mut res2 = 0;

    for c in re2.captures_iter(&parsed) {
        if &c[0] == "do()" {
            on = true;
        } else if &c[0] == "don't()" {
            on = false;
        } else {
            let prod = c[1].parse::<u32>().expect(&c[1]) * c[2].parse::<u32>().expect(&c[2]);
            res1 += prod;
            if on {
                res2 += prod;
            }
        }
    }
    (res1, res2)
}
fn main() {
    let mut input = String::new();
    
    let _ = io::stdin().read_to_string(&mut input);
    use std::time::Instant;
    let now = Instant::now();

    let (r1, r2) = compute(input);
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
        let input =
            String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(compute(input).0, 161);
    }
    #[test]
    fn example2() {
        // avoid parsing issues
        let input = String::from(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        );
        assert_eq!(compute(input).1, 48);
    }
}
