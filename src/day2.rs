use std::io::{self, BufRead};
fn main() {
    let stdin = io::stdin();
    let _parsed = stdin.lock().lines();
}