use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .ok()
        .expect("Failed to read line");

    let n: i32 = input.trim().parse().unwrap();

    for _ in 0..n {
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .ok()
            .expect("Failed to read line");
        //let input = input.split(' ').collect();
        let input: Vec<_> = input.trim().split(' ').collect();
        let mut x = input[0].parse::<i32>().unwrap();
        let mut y = input[1].parse::<i32>().unwrap();

        let mut ret = 0;
        let mut screens = 0;
        if y % 2 == 0 {
            ret = y / 2;
            screens = ret;
            while x > 0 && screens > 0 {
                screens -= 1;
                x -= 7;
            }
            while x > 0 {
                x -= 15;
                ret += 1;
            }
        } else {
            ret = y / 2 + 1;
            screens = ret;
            let mut left = x - screens * 7 - 4;
            if left <= 0 {
            } else {
                while left > 0 {
                    left -= 15;
                    ret += 1;
                }
            }
        }

        println!("{}", ret);
    }
}
