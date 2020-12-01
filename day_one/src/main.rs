fn main() {
    let input = include_str!("../input").lines().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    for (i, a) in input.iter().enumerate(){
        for b in input.iter().skip(i) {
            if a + b == 2020 {
                println!("Found sum of two which == 2020 ({} + {}). Result is {}", a, b, a * b );

            }
            let _res = input.iter().cloned().skip(i+1)
                .filter(|c| a + b + c == 2020)
                .take(1)
                .inspect(|c| println!("Found sum of three which == 2020 ({} + {} + {}). Result is {}", a, b, c,  a * b * c ))
                .collect::<Vec<_>>();
            
        }
    }
}
