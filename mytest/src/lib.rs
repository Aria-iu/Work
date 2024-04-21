use std::fmt::Display;


//https://c03w331xxs3.feishu.cn/wiki/Fnapw361oi6IW5koCXBcvLK0nhg
trait foo {
    fn method<T>(&self, t: T);
}

struct Bar;

impl Bar {
    fn new() -> Self {
        Self {}
    }
}

impl foo for Bar {
    fn method<T>(&self, t: T) {
        println!("Bar impl trait foo!");
    }
}

trait Foo {
    type T: Display;
    fn method(&self,t:Self::T);
}

struct Bar2;

impl Bar2{
    fn new()->Self{
        Self {}
    }
}

impl Foo for Bar2{
    type T = u8;
    fn method(&self, t: Self::T) {
        println!("inner type value is {}" , t);
    }
}

macro_rules! capture_then_what_is {
    (#[$m:meta]) => {what_is!(#[$m])};
}

macro_rules! what_is {
    (#[no_mangle]) => {"no_mangle attribute"};
    (#[inline]) => {"inline attribute"};
    (#[$m:meta]) => {"you got me!"};
    ($($tts:tt)*) => {concat!("something else (", stringify!($($tts)*), ")")};
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::VacantEntry;

    #[test]
    fn as_trait_bound() {
        let bar = Bar::new();
        bar.method(0u8);
    }
/*
    #[test]
    fn as_trait_obj() {
        let bar = Bar::new();
        let mut v: Vec<&dyn foo> = vec![];
        v.push(&bar);
    }
 */

    #[test]
    fn as_tarit_bound2(){
        let bar2 = Bar2::new();
        bar2.method(8);
    }

    #[test]
    fn as_trait_object2(){
        let bar2 = Bar2::new();
        let mut v: Vec<&dyn Foo<T=u8>> = vec![];
        v.push(&bar2);
    }

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
}

