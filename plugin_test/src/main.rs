extern crate plugin_demo;
use plugin_demo::roman_to_arabic_macro;

fn main() {
    let number = roman_to_arabic_macro!("XIV");
    println!("The Arabic number is: {}", number);
}
