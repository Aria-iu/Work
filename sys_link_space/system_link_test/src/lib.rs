extern crate system_math_example;

#[cfg(test)]
mod system_link_tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = system_math_example::square_root(4.0);
        assert_eq!(result, 2.0);
    }
}
