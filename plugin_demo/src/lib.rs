extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

fn roman_to_arabic(roman: &str) -> u64 {
    let mut result = 0;
    let mut prev_value = 0;

    for ch in roman.chars().rev() {
        let value = match ch {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => panic!("Invalid Roman numeral"),
        };

        if value < prev_value {
            result -= value;
        } else {
            result += value;
        }

        prev_value = value;
    }

    result
}

#[proc_macro]
pub fn roman_to_arabic_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let roman = input.value();
    let arabic = roman_to_arabic(&roman);

    let expanded = quote! {
        #arabic
    };

    TokenStream::from(expanded)
}

