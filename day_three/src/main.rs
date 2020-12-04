use std::io::{stdin, BufRead};

fn main() {
    let step_r = [1,3,5,7,1];
    let step_d = [1,1,1,1,2];
    let input = stdin().lock()
        .lines()
        .map(|x| x.unwrap().bytes().map(|x| x == b'#').collect::<Vec<_>>())
        .collect::<Vec<Vec<bool>>>();

    let ans = step_r.iter()
        .enumerate()
        .map(|(i,x)| {
            input.iter()
                .enumerate()
                .step_by(step_d[i])
                .map(|(zi, y)| y[(*x * zi / step_d[i]) % y.len()] as usize)
                .sum()
        })
        .collect::<Vec<usize>>();
    println!("Part One: {}\nPart Two: {}", ans[1], ans.iter().product::<usize>());
}
