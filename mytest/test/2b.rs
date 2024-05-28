use std::cmp::min;
use std::io::stdin;

fn main() {
    let mut dp = vec![ vec![vec![0u32 ;2]; 1010]; 1010];
    let mut ans = Vec::new();
    let mut flag = -1;
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .ok()
        .expect("Failed to read line");

    let n: i32 = input.trim().parse().unwrap();

    for i in 0..n{
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .ok()
            .expect("Failed to read line");
        let input: Vec<_> = input.trim().split(' ').collect();
        let mut j: u32 = 0;
        for s in input{
            let mut t = s.parse::<u32>().unwrap();
            if t == 0{
                flag = i;
                continue;
            }
            while (t%2==0) {
                t/=2;
                dp[i as usize][j as usize][0] += 1;
            }
            while(t%5==0){
                t/=5;
                dp[i as usize][j as usize][1]+=1;
            }
            j += 1;
        }
    }



    for i in 1..n{
        dp[i as usize][0][0] += dp[i as usize-1usize][0][0];
        dp[i as usize][0][1] += dp[i as usize-1usize][0][1];
        dp[0][i as usize][0] += dp[0][i as usize -1usize][0];
        dp[0][i as usize][1] += dp[0][i as usize -1usize][1];
    }

    for i in 1..n{
        for j in 1..n{
            dp[i as usize][j as usize][0] += min(dp[i as usize-1usize][j as usize][0],dp[i as usize][j as usize-1usize][0] );
            dp[i as usize][j as usize][1] += min(dp[i as usize-1usize][j as usize][1],dp[i as usize][j as usize-1usize][1] );
        }
    }

    let res;
    if dp[(n-1) as usize][(n-1)as usize][0]<dp[(n-1) as usize][(n-1)as usize][1]{
        res = 0;
    }else{
        res = 1;
    }
/*
    for i in 0..n{
        for j in 0..n{
            print!("{},{} ",dp[i as usize][j as usize][0],dp[i as usize][j as usize][1]);
        }
        println!(" ");
    }
*/
    if (flag!=-1 && dp[n as usize-1usize][n as usize-1usize][res]>=1){
        println!("1");
        for i in 0..flag{
            ans.push("D");
        }
        for i in 0..n-1{
            ans.push("R");
        }
        for i in flag..n-1{
            ans.push("D");
        }
        for i in ans.iter(){
            print!("{}",i);
        }
        return;
    }

    let mut p1 = n-1;
    let mut p2 = n-1;
    while (p1!=0&&p2!=0){
        if dp[p1 as usize - 1usize][p2 as usize][res]<dp[p1 as usize ][p2 as usize- 1usize][res]{
            p1 -= 1;
            // reverse
            ans.push("D");
        }else{
            p2-=1;
            // reverse
            ans.push("R");
        }
    }
    while(p1!=0){
        ans.push("D");
        p1-=1;
    }
    while(p2!=0){
        ans.push("R");
        p2-=1;
    }
    println!("{}",dp[n as usize-1usize][n as usize-1usize][res]);
    for i in ans.iter().rev(){
        print!("{}",i);
    }

    return ;
}