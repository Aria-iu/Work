use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .ok()
        .expect("Failed to read line");

    let n: i32 = input.trim().parse().unwrap();

    for _ in 0..n{
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .ok()
            .expect("Failed to read line");
        let input: Vec<_> = input.trim().split(' ').collect();
        let a = input[0].parse::<i32>().unwrap();
        let b = input[1].parse::<i32>().unwrap();

        if a==b {
            println!("0");
        }else if a<b {
            let d = b-a;
            if d%2 == 0 {
                if (d/2)%2==1{
                    println!("2");
                }else{
                    println!("3");
                }
            }else{
                println!("1");
            }
        }else{
            let d = a-b;
            if d%2==1{
                println!("2");
            }else{
                println!("1");
            }
        }

    }

}