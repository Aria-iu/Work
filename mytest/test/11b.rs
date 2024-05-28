use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .ok()
        .expect("Failed to read line");
    let input: Vec<_> = input.trim().split(' ').collect();
    let mut x = input[0].parse::<i64>().unwrap();
    if x<0{
        x = -x;
    }

    let mut cnt = 0;
    let mut temp = 0;
    while temp<x||(temp-x)%2!=0 {
        cnt+=1;
        temp+=cnt;
    }
    println!("{}",cnt);
}