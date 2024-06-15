use std::collections::{HashMap, HashSet};
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
        let len: i32 = input.trim().parse().unwrap();
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .ok()
            .expect("Failed to read line");
        let letters: String = input.trim().parse::<String>().unwrap();
        // println!("letter is : {}",letters);
        // 计算映射关系
        let mut que = Vec::new();
        let mut set: HashSet<char> = HashSet::new();
        for i in letters.as_str().bytes() {
            set.insert(i as char);
        }
        let len = set.len();
        for t in set {
            que.push(t);
        }
        que.sort();
        /*
              for i in que{
                  print!("{} ",i);
              }
              println!(" ");
              println!(" ");
        */
        /*
        let mut temp = Vec::new();
        for i in que.iter().rev(){
            temp.push(*i);
        }
         */

        let mut iter1 = que.iter();
        let mut iter2 = que.iter().rev();
        let mut mapper: HashMap<char, char> = HashMap::new();
        for _ in 0..len {
            let a = iter1.next().unwrap();
            let b = iter2.next().unwrap();
            mapper.insert(*a, *b);
        }
        /*
                for (a,b) in mapper{
                    println!("{} -> {}",a,b);
                }
        */
        // decode
        let mut result = String::new();
        for i in letters.as_str().bytes() {
            let ch = i as char;
            let temp = mapper.get(&ch).unwrap();
            result.push(*temp);
        }
        println!("{}", result);
    }
}
