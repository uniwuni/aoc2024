use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

fn is_valid(rules: &HashMap<u32, Vec<u32>>, update: &[u32]) -> bool {
    let mut s: HashSet<u32> = HashSet::new();
    let full: HashSet<&u32> = update.iter().collect();
    for n in update {
        if let Some(a) = rules.get(&n) {
            for precond in a {
                if !s.contains(precond) && full.contains(precond) {
                    return false;
                }
            }
        }
        s.insert(*n);
    }
    true
}

fn make_valid(rules: &HashMap<u32, Vec<u32>>, update: &[u32]) -> Vec<u32> {
    let mut s: HashSet<u32> = HashSet::new();
    let full: HashSet<&u32> = update.iter().collect();
    let mut ordered: Vec<u32> = vec![];
    let mut stack: Vec<u32> = update.to_vec();
    stack.reverse();
    while let Some(n) = stack.pop() {
        let mut unresolved = false;
        if s.contains(&n) {
            continue;
        }
        if let Some(dep_vec) = rules.get(&n) {
            for dep in dep_vec {
                if !s.contains(dep) && full.contains(dep) {
                    if !unresolved {
                        stack.push(n);
                    }
                    unresolved = true;
                    stack.push(*dep);
                }
            }
        }
        if !unresolved {
            ordered.push(n);
            s.insert(n);
        }
    }
    ordered
}

fn compute(rules: HashMap<u32, Vec<u32>>, updates: &[Vec<u32>]) -> (u32, u32) {
    /* day 1 */
    let (ordered, notordered): (Vec<_>, Vec<_>) =
        updates.iter().partition(|&x| is_valid(&rules, &x));
    let result1: u32 = ordered.iter().map(|x| x[x.len() / 2]).sum();
    let result2: u32 = notordered
        .iter()
        .map(|x| make_valid(&rules, x)[x.len() / 2])
        .sum();

    /* day 2 */
    (result1, result2)
}
fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let mut empty_hit = false;
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut updates: Vec<Vec<u32>> = vec![];
    for line in lines {
        let line = line.unwrap();
        if !empty_hit {
            if line.is_empty() {
                empty_hit = true;
            } else {
                let Some((l1, l2)) = line.split_once('|') else {
                    todo!("wah")
                };
                let n1: u32 = l1.parse().unwrap();
                let n2: u32 = l2.parse().unwrap();
                if let Some(v) = rules.get_mut(&n2) {
                    v.push(n1);
                } else {
                    rules.insert(n2, vec![n1]);
                }
            }
        } else {
            updates.push(line.split(',').map(|s| s.parse::<u32>().unwrap()).collect());
        }
    }

    use std::time::Instant;
    let now = Instant::now();

    let (r1, r2) = compute(rules, &updates);
    let elapsed = now.elapsed();
    println!("{} {}", r1, r2);

    println!("Elapsed: {:.2?}", elapsed);
}
