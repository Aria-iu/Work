use std::collections::HashMap;
use std::io::stdin;
use std::vec::Vec;

fn main() {
    let mut ns_map: HashMap<String, Vec<i32>> = HashMap::new();

    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .ok()
        .expect("Failed to read line");

    let n: i32 = input.trim().parse().unwrap();

    for i in 0..n {
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .ok()
            .expect("Failed to read line");
        let input: Vec<_> = input.trim().split(' ').collect();
        let name = input[0].parse::<String>().unwrap();
        let val = input[1].parse::<i32>().unwrap();

        if !ns_map.contains_key(&name) {
            let mut score: Vec<i32> = Vec::new();

            if i == 0 {
                score.push(val);
            } else {
                for _ in 0..i {
                    score.push(0);
                }
                score.push(val);
            }
            ns_map.insert(name.clone(), score);
            for (t_name, t_score) in &mut ns_map {
                if *t_name != name {
                    let num = t_score.last().unwrap() + 0;
                    t_score.push(num);
                }
            }
        } else {
            for (t_name, score) in &mut ns_map {
                if *t_name != name {
                    let num = score.last().unwrap() + 0;
                    score.push(num);
                } else {
                    let num = score.last().unwrap() + val;
                    score.push(num);
                }
            }
        }
    }

    let mut max = 0;
    let mut pres = Vec::new();
    for (_, score) in &ns_map {
        let t = score.last().unwrap();
        if *t > max {
            max = *t;
        }
    }
    for (name, score) in &ns_map {
        let t = score.last().unwrap();
        if *t == max {
            pres.push(name);
        }
    }
    'outer: for i in 0..n {
        for j in &pres {
            let temp = ns_map.get(*j).unwrap();
            let num = temp.get(i as usize).unwrap();
            if *num >= max {
                print!("{}", j);
                break 'outer;
            }
        }
    }
}
