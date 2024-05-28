use std::io::stdin;
use std::vec::Vec;

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
        let len: i32 = input.trim().parse().unwrap();
        if len==3{
            println!("0");
            continue;
        }
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .ok()
            .expect("Failed to read line");
        let letters: Vec<_> = input.trim().split(' ').collect();
        let mut nums = Vec::new();
        for t in 0..len{
            let num = letters[t as usize].parse::<i32>().unwrap();
            nums.push(num);
        }

        let bound = len-3;
        let mut result = 0;
        for i in 0..bound+1{
            // 以i开头的数组
            for j in i+1..bound+1{
                // 以j开头的数组
                // 判断以i开头的数组和以j开头的数组是否满足要求
                if nums[i as usize]==nums[j as usize]&&nums[i as usize+ 1usize]==nums[j as usize+1usize]&&nums[i as usize+2usize]!=nums[j as usize+2usize]||
                    nums[i as usize]==nums[j as usize]&&nums[i as usize+ 1usize]!=nums[j as usize+1usize]&&nums[i as usize+2usize]==nums[j as usize+2usize]||
                        nums[i as usize]!=nums[j as usize]&&nums[i as usize+ 1usize]==nums[j as usize+1usize]&&nums[i as usize+2usize]==nums[j as usize+2usize]
                {
                    result+=1;
                }
            }
        }

        println!("{}",result);


    }

}
