use std::cmp::Ordering;
use std::io::stdin;

#[derive(Debug)]
pub struct Letter {
    width: u32,
    height: u32,
    index: u32,
}
fn main() {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .ok()
        .expect("Failed to read line");
    let input: Vec<_> = input.trim().split(' ').collect();
    let mut letter_n = input[0].parse::<u32>().unwrap();
    let card_w = input[1].parse::<u32>().unwrap();
    let card_h = input[2].parse::<u32>().unwrap();

    let mut letters = Vec::new();

    for i in 0..letter_n {
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .ok()
            .expect("Failed to read line");
        let input: Vec<_> = input.trim().split(' ').collect();
        let temp1 = input[0].parse::<u32>().unwrap();
        let temp2 = input[1].parse::<u32>().unwrap();
        if temp1 > card_w && temp2 > card_h {
            let temp_letter = Letter {
                width: temp1,
                height: temp2,
                index: i + 1,
            };
            letters.push(temp_letter);
        } else {
            letter_n -= 1;
        }
    }
    /*
        for i in &letters{
            println!("{:?}",i);
        }
    */
    // sort by width, dp by height
    if letters.len() == 0 {
        print!("0");
        return;
    }

    letters.sort_by(|a, b| a.width.cmp(&b.width));
    let mut pre: Vec<i32> = vec![-1; letter_n as usize];
    let mut max = 0;
    let mut max_index: i32 = 0;
    let mut dp = vec![0; letter_n as usize];
    for i in 0..letter_n {
        dp[i as usize] = 1;
        for j in 0..i {
            if letters[i as usize].height > letters[j as usize].height
                && letters[i as usize].width > letters[j as usize].width
                && dp[j as usize] + 1 > dp[i as usize]
            {
                dp[i as usize] = dp[j as usize] + 1;
                pre[i as usize] = j as i32;
            }
        }
        if dp[i as usize] > max {
            max = dp[i as usize];
            max_index = i as i32;
        }
    }

    println!("{}", dp[max_index as usize]);
    let mut ans = Vec::new();
    let mut ind = 0;
    while (max_index != -1) {
        ans.push(letters[max_index as usize].index);
        ind += 1;
        max_index = pre[max_index as usize];
    }

    for iter in ans.iter().rev() {
        print!("{}", iter);
        ind -= 1;
        if ind == 0 {
            print!("\n");
            return;
        }
        print!(" ");
    }
}
