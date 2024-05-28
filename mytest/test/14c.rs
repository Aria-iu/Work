use std::io::stdin;

struct Point{
    x: i32,
    y: i32,
}
struct Line{
    p1: Point,
    p2: Point,
}
impl Line {
    // 计算线段长度的平方
    fn length_squared(&self) -> i32 {
        let dx = self.p2.x - self.p1.x;
        let dy = self.p2.y - self.p1.y;
        dx * dx + dy * dy
    }

    fn is_vertical_x(&self)->bool{
        self.p1.x == self.p2.x
    }
    fn is_vertical_y(&self)->bool{
        self.p1.y == self.p2.y
    }
    // 计算向量
    fn vector(&self) -> (i32, i32) {
        (self.p2.x - self.p1.x, self.p2.y - self.p1.y)
    }

    fn is_satisfy(&self) -> bool{
        self.p1.x==self.p2.x||self.p1.y==self.p2.y
    }

    // 判断两条线段是否垂直
    fn is_perpendicular(&self, other: &Line) -> bool {
        let self_vector = self.vector();
        let other_vector = other.vector();
        // 两向量的点积为0时，表示两条线段垂直
        self_vector.0 * other_vector.0 + self_vector.1 * other_vector.1 == 0
    }
}

fn main() {

    let mut L = Vec::new();

    for i in 0..4{
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .ok()
            .expect("Failed to read line");
        let input: Vec<_> = input.trim().split(' ').collect();
        let temp_x1 = input[0].parse::<i32>().unwrap();
        let temp_y1 = input[1].parse::<i32>().unwrap();
        let temp_x2 = input[2].parse::<i32>().unwrap();
        let temp_y2 = input[3].parse::<i32>().unwrap();
        let p1 = Point{
            x: temp_x1,
            y: temp_y1,
        };
        let p2 = Point{
            x: temp_x2,
            y: temp_y2,
        };
        let t_L = Line{
            p1,
            p2,
        };
        L.push(t_L);
    }

    let l1 = L.get(0).unwrap();
    let l2 = L.get(1).unwrap();
    let l3 = L.get(2).unwrap();
    let l4 = L.get(3).unwrap();
    {
        // 边不为0,且两两长度相等
        if l1.length_squared()==0||l2.length_squared()==0||l3.length_squared()==0||l4.length_squared()==0{
            println!("NO");
            return;
        }
        if (l1.length_squared()==l2.length_squared()&&l3.length_squared()==l4.length_squared())
            ||(l1.length_squared()==l3.length_squared()&&l2.length_squared()==l4.length_squared())
            || (l1.length_squared()==l4.length_squared()&&l2.length_squared()==l3.length_squared())
        {

        }else{
            println!("NO");
            return;
        }
    }

    {
        // 检查两个垂直
        let mut cnt_x = 0;
        let mut cnt_y = 0;
        for i in L.iter() {
            if i.is_vertical_x() {
                cnt_x += 1;
            } else {
                if i.is_vertical_y() {
                    cnt_y += 1;
                }
            }
        }
        if cnt_x == 2 && cnt_y == 2 {} else {
            println!("NO");
            return;
        }
    }

    // 检查4条边是否满足一个矩形。
    {

    }
}