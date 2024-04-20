// 慎重使用：在添加字段或变体时，增加主版本通常是更好的选择。 
// #[non_exhaustive] 可能适用于这样的
//情况：你正在为一个可能与你的库不同步变化的外部资源建模，但这不是一个通用工具。
mod a {
    // Public struct.
    #[non_exhaustive]
        pub struct S {
        pub foo: i32,
    }
    #[non_exhaustive]
    pub enum AdmitMoreVariants {
        VariantA,
        VariantB,
        #[non_exhaustive]
        VariantC { a: String }
    }
}

fn print_matched_variants(s: a::S) {
    // Because S is `#[non_exhaustive]`, it cannot be named here and
    // we must use `..` in the pattern.
    let a::S { foo: _, ..} = s;

    let some_enum = a::AdmitMoreVariants::VariantA;
    match some_enum {
        a::AdmitMoreVariants::VariantA => println!("it's an A"),
        a::AdmitMoreVariants::VariantB => println!("it's a b"),
        // .. required because this variant is non-exhaustive as well
        a::AdmitMoreVariants::VariantC { a, .. } => println!("it's a c"),
        // The wildcard match is required because more variants may be
        // added in the future
        _ => println!("it's a new variant")
    }
}