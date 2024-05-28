#![feature(test)]
#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate type_name_derive;
trait TypeName {
    fn type_name() -> &'static str;
}

mod mio;
use mio::test1_1;

macro_rules! capture_then_what_is {
    (#[$m:meta]) => {what_is!(#[$m])};
}

macro_rules! what_is {
    (#[no_mangle]) => {"no_mangle attribute"};
    (#[inline]) => {"inline attribute"};
    (#[$m:meta]) => {"you got me!"};
    ($($tts:tt)*) => {concat!("something else (", stringify!($($tts)*), ")")};
}
/*

// pass copy
/*
test benches::bench_fab_0  ... bench:           9 ns/iter (+/- 0)
test benches::bench_fab_1  ... bench:          10 ns/iter (+/- 0)
test benches::bench_fab_10 ... bench:         315 ns/iter (+/- 6)
test benches::bench_fab_2  ... bench:          13 ns/iter (+/- 0)
test benches::bench_fab_20 ... bench:      39,655 ns/iter (+/- 1,706)
 */
pub fn fab(n: u32) -> u32{
    if n==0 || n==1{
       n 
    }else{
        fab(n-1)+fab(n-2)
    }
}
*/
/*
test benches::bench_fab_0  ... bench:           8 ns/iter (+/- 1)
test benches::bench_fab_1  ... bench:           8 ns/iter (+/- 0)
test benches::bench_fab_10 ... bench:           9 ns/iter (+/- 0)
test benches::bench_fab_2  ... bench:           9 ns/iter (+/- 0)
test benches::bench_fab_20 ... bench:          10 ns/iter (+/- 0)
*/
pub fn fab(n: u32) -> u32{
    if n==0||n==1{
        n
    }else{
        let mut pre = 1;
        let mut cur = 1;
        for _ in 2..n{
            let new_cur = pre+cur;
            pre = cur;
            cur = new_cur;
        }
        cur
    }
}


#[cfg(test)]
mod tests {

    use log::{debug, error, info, trace};
    use super::*;

    #[test]
    fn macro_test()
    {
        println!(
            "{}\n{}\n{}\n{}",
            what_is!(#[no_mangle]),
            what_is!(#[inline]),
            capture_then_what_is!(#[no_mangle]),
            capture_then_what_is!(#[inline]),
        );
    }


    #[test]
    fn it_fab() {
        assert_eq!(fab(0),0);
        assert_eq!(fab(1),1);
        assert_eq!(fab(2),1);
        assert_eq!(fab(10),55);
        assert_eq!(fab(20),6_765);
    }

    #[test]
    fn test_log(){
        env_logger::init();
        trace!("Logging {} small thing(s)", 1);
        debug!("Some debug information: {}","the answer is 42");
        info!("This is an interesting information");
        error!("An error happened, do something!");
    }

    #[test]
    fn mio_test1(){
        let ret = test1_1();
        match ret {
            Ok(()) => {info!("OK");},
            Err(_) => {error!("Wrong");panic!("wrong");},
        }
    }

    #[derive(TypeName)]
    struct Alice;
    #[derive(TypeName)]
    enum Bob {}
    #[test]
    fn proc_macro_test(){
        println!("Alice's name is {}", Alice::type_name());
        println!("Bob's name is {}", Bob::type_name());
    }


    #[derive(Debug, Getters, Setters)]
    struct Alice2 {
        x: String,
        y: u32,
    }
    #[test]
    fn proc_complex_test(){
        let mut alice = Alice2 {
            x: "this is a name".to_owned(),
            y: 34
        };
        println!("Alice2: {{ x: {}, y: {} }}",
                 alice.x(),
                 alice.y());
        alice.set_x("testing str");
        alice.set_y(15u8);
        println!("{:?}", alice);
    }

    #[test]
    fn just_test(){
        let s = String::from("hello");
        //let s2;
        let b = false;
        if b{
            let s2 = s;
        }else{
            println!("{}",s);
        }

    }
}

