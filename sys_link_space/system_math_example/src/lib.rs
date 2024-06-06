use std::os::raw::c_double;

extern {
    fn sqrt(x: c_double) -> c_double;
}

// 一个rust包装器，用来调用外部函数
pub fn square_root(x: f64) -> f64{
    unsafe{
        sqrt(x)
    }
}

#[cfg(test)]
mod system_math_example_tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(square_root(9.0),3.0);
        assert_eq!(square_root(16.0),4.0);
        assert_eq!(square_root(25.0),5.0);
    }
}
