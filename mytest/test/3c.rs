use std::io::stdin;

fn main() {
    let mut num: Vec<Vec<i8>> = vec![vec![-1; 3]; 3];
    for i in 0i8..3i8 {
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .ok()
            .expect("Failed to read line");
        let input: &str = input.trim();
        let temp1 = input.as_bytes()[0];
        let temp2 = input.as_bytes()[1];
        let temp3 = input.as_bytes()[2];
        //println!("{} {} {}",temp1 as char,temp2 as char,temp3 as char);
        if (temp1 == '.' as u8) {
        } else if (temp1 == '0' as u8) {
            num[i as usize][0] = 0;
        } else {
            num[i as usize][0] = 1;
        }
        if (temp2 == '.' as u8) {
        } else if (temp2 == '0' as u8) {
            num[i as usize][1] = 0;
        } else {
            num[i as usize][1] = 1;
        }
        if (temp3 == '.' as u8) {
        } else if (temp3 == '0' as u8) {
            num[i as usize][2] = 0;
        } else {
            num[i as usize][2] = 1;
        }
    }

    let mut o_num = 0;
    let mut x_num = 0;
    for i in 0..3 {
        for j in 0..3 {
            if num[i][j] == 0 {
                o_num += 1;
            } else if num[i][j] == 1 {
                x_num += 1;
            }
        }
    }
    if ((o_num - x_num) >= 1 || (o_num - x_num) < -1) {
        println!("illegal");
        return;
    }

    for i in 0..3 {
        if (num[i][0] != -1 && (num[i][0] == num[i][1] && num[i][1] == num[i][2])) {
            if num[i][0] == 0 {
                if x_num > o_num {
                    println!("illegal");
                    return;
                }
                println!("the second player won");
                return;
            } else {
                if o_num >= x_num {
                    println!("illegal");
                    return;
                }
                println!("the first player won");
                return;
            }
        }
        if (num[0][i] != -1 && (num[0][i] == num[1][i] && num[1][i] == num[2][i])) {
            if num[0][i] == 0 {
                if x_num > o_num {
                    println!("illegal");
                    return;
                }
                println!("the second player won");
                return;
            } else {
                if o_num >= x_num {
                    println!("illegal");
                    return;
                }
                println!("the first player won");
                return;
            }
        }
    }
    if (num[0][0] != -1 && (num[0][0] == num[1][1] && num[1][1] == num[2][2])) {
        if num[0][0] == 0 {
            if x_num > o_num {
                println!("illegal");
                return;
            }
            println!("the second player won");
            return;
        } else {
            if o_num >= x_num {
                println!("illegal");
                return;
            }
            println!("the first player won");
            return;
        }
    }
    if (num[0][2] != -1 && (num[0][2] == num[1][1] && num[1][1] == num[2][0])) {
        if num[0][2] == 0 {
            if x_num > o_num {
                println!("illegal");
                return;
            }
            println!("the second player won");
            return;
        } else {
            if o_num >= x_num {
                println!("illegal");
                return;
            }
            println!("the first player won");
            return;
        }
    }
    if o_num + x_num != 9 {
        if o_num > x_num {
            println!("illegal");
            return;
        } else if o_num == x_num {
            println!("first");
            return;
        } else {
            println!("second");
            return;
        }
    } else {
        println!("draw");
        return;
    }
}
