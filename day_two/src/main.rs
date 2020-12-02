use std::collections::HashMap;
use std::io::{stdin, BufRead};
fn main() {
    let (mut p1, mut p2) = (0,0);
        include_str!("../input")
        .lines()
        .map(|x| x.split(&['-', ' ', ':'][..])
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>())
        .map(|x| (x[0].parse::<usize>().unwrap(), x[1].parse::<usize>().unwrap(), x[2].chars().next().unwrap(), x[3]))
        .for_each(|(b, a, key, pass)| {
            let n = n_occurrences(key, pass);
            if b <= n && n <= a {
                p1 += 1;
            }
            let hash = pass.char_indices().collect::<HashMap<_,_>>();
            if !(hash.get(&(a - 1)).unwrap() == &key) != !(hash.get(&(b - 1)).unwrap() == &key) {
                p2 += 1;
            }
        });
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn n_occurrences(key: char, pass: &str) -> usize {
    let mut ctr: usize = 0;
    for ch in pass.chars() {
        if ch == key {
            ctr += 1;
        }
    }
    return ctr
} 
