use std::io::stdin;
use std::vec::Vec;

fn main() {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .ok()
        .expect("Failed to read line");

    let mut n: i32 = input.trim().parse().unwrap();

    while n>0 {
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .ok()
            .expect("Failed to read line");

        let len: i32 = input.trim().parse().unwrap();
        let mut a = vec![0;0];
        let mut b = vec![0;0];

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .ok()
            .expect("Failed to read line");
        let input: Vec<_> = input.trim().split(' ').collect();
        for iter in input.iter(){
            let val  = iter.parse::<u32>().unwrap();
            a.push(val);
        }

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .ok()
            .expect("Failed to read line");
        let input: Vec<_> = input.trim().split(' ').collect();
        for iter in input.iter(){
            let val  = iter.parse::<u32>().unwrap();
            b.push(val);
        }
/*
        for i in a{
            println!("{}",i);
        }
        for i in b{
            println!("{}",i);
        }
*/
        let mut pa = 0;
        let mut pb = 0;
        let mut ans = 0;

        while(pa!=len&&pb!=len){
            let temp_a = a.get(pa as usize).unwrap();
            let temp_b = b.get(pb as usize).unwrap();
            if *temp_a > *temp_b{
                ans += 1;
                pb +=1;
            }else{
                pa+=1;
                pb+=1;
            }
        }

        println!("{}",ans);
        n-=1;
    }
    

}