/*
如果我们使用枚举，我们可以解构不同的内部类型
可以解构结构体来提取特定的值，甚至可以在结构体中的特定值上进行匹配
*/
enum DistinctTypes {
    Name(String),
    Count(i32),
}

fn match_enum_types(enum_types: &DistinctTypes) {
    match enum_types {
        DistinctTypes::Name(name) => println!("name={name}"),
        DistinctTypes::Count(count) => println!("count={count}"),
    }
}


/*
Pattern matching 结合 ? 操作符是处理函数返回 Result 或 Option 的错误的一种清晰方式。
通过在 Result 或 Option 返回错误或 None 时立即返回，
我们可以使用 ? 操作符将代码的嵌套程度大大降低，从而提高可读性。
*/
fn write_to_file() -> std::io::Result<()> { 
    use std::fs::File;
    use std::io::prelude::*;
    
    let mut file = File::create("filename")?; 
    file.write_all(b"File contents")?; 
    Ok(()) 
}

fn try_to_write_to_file() {
    match write_to_file() {
        Ok(()) => println!("Write suceeded"),
        Err(err) => println!("Write failed: {}", err.to_string()),
    }
}
// 与上面的等价
fn write_to_file_without_result() {
    use std::fs::File;
    use std::io::prelude::*;
    
    {
        let create_result = File::create("filename");
        match create_result {
            Ok(mut file) => match file.write_all(b"File contents") {
                Err(err) => { println!("There was an error writing: {}", err.to_string()) }
                _ => println!("Write suceeded"),
            },
            Err(err) => println!("There was an error opening the file: {}", err.to_string()),
        }
    }
}

