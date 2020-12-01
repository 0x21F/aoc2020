
fn main() {
    let input = include_str!("../input").lines().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let (x, y) = calc(input);
    println!("Result for part one is {}", x);
    println!("Result for part two is {}", y); 
}

fn calc(input: Vec<i32>) -> (i32, i32) {
    let (mut x, mut y): (i32, i32) = (0,0);
    let mut check = false;
    for (i, a) in input.iter().enumerate(){
        for b in input.iter().skip(i) {
            if a + b == 2020{
                 x = a * b;
            }
            if check {
                break;
            }
            
            for c in input.iter().skip(i+1) {
                if a + b + c == 2020 {
                    y = a * b * c; 
                    check = true;
                    break;
                }

            }
        }
    }
    return (x,y)
}
