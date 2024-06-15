

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        use blocking::unblock;
        use std::fs;

        let content = unblock(|| fs::read_to_string("./file.txt"));
        let res = futures::executor::block_on(content);

        if res.is_ok(){
            // println!("content is {:?}", res);
            assert_eq!("abc",res.unwrap());
        }else{
            panic!("read failed");
        }
    }

    #[test]
    fn test2(){
        use blocking::{unblock, Unblock};
        use futures::io;
        use std::fs::File;

        let input = unblock(|| File::open("file.txt"));
        let res = futures::executor::block_on(input).unwrap();
        let input = Unblock::new(res);
        let mut output = Unblock::new(std::io::stdout());

        let fut = io::copy(input, &mut output);
        let res = futures::executor::block_on(fut).unwrap();
        println!("copy text num is {}",res);
    }
}
