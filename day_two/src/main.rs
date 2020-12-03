use std::io::{stdin, Read};
use rayon::str::ParallelString;
use rayon::iter::ParallelIterator;


fn main() {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();

    let (part_one, part_two) = input.as_parallel_string()
        .par_lines()
        .map(|y| {
            let x = y.split(&['-', ' ', ':'][..]).filter(|s| !s.is_empty()).collect::<Vec<_>>();

            let b = x[0].parse::<usize>().unwrap();
            let a = x[1].parse::<usize>().unwrap();
            let key = x[2].chars().next().unwrap();
            let pass = x[3];
            let n = pass.matches(key).count();

            ((b <= n && n <= a) as usize,
            ((pass.chars().nth(a - 1).unwrap() == key) ^ (pass.chars().nth(b - 1).unwrap() == key)) as usize)
        })
        .fold(|| (0, 0), |z, (x, y)| {
            (z.0 + x,
            z.1 + y)
        })
        .reduce(||(0, 0), |a, (x,y)| (a.0 + x, a.1 + y));
    println!("part one: {} \npart two: {}", part_one, part_two);
}
