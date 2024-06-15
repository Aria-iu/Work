use plugin_demo::roman_to_arabic_macro;


fn main(){
    let num = roman_to_arabic_macro!("XIV");
    println!("The Arabic num is: {}",num);
}
