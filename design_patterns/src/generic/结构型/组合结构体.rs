/*
    将该结构体分解为几个较小的结构体。 然后将这些结构体组合为原始结构体。 
    然后每个结构体都可以被单独借用，并具有更灵活的行为。
    满足借用检查器
*/
#![allow(unused)]
fn main() {
// A is now composed of two structs - B and C.
struct A {
    b: B,
    c: C,
}
struct B {
    f2: u32,
}
struct C {
    f1: u32,
    f3: u32,
}

// These functions take a B or C, rather than A.
fn foo(b: &mut B) -> &u32 { &b.f2 }
fn bar(c: &mut C) -> u32 { c.f1 + c.f3 }

fn baz(a: &mut A) {
    let x = foo(&mut a.b);
    // Now it's OK!
    let y = bar(&mut a.c);
    println!("{}", x);
}
}
