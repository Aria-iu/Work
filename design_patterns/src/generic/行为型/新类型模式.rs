/*
在某些情况下，我们希望一个类型的行为类似于另一个类型，
或者在编译时强制执行一些行为，而仅仅使用类型别名是不够的

可以使用Newtype模式来提供类型安全和封装
*/

// Bar是Foo的新类型（单元组类型），可以绕过孤儿原则

// Some type, not necessarily in the same module or even crate.
struct Foo {
    //..
}

impl Foo {
    // These functions are not present on Bar.
    //..
}

// The newtype.
pub struct Bar(Foo);

impl Bar {
    // Constructor.
    pub fn new(
        //..
    ) -> Self {

        //..

    }

    //..
}

fn main() {
    let b = Bar::new(/*...*/);

    // Foo and Bar are type incompatible, the following do not type check.
    // let f: Foo = b;
    // let b: Bar = Foo { ... };
}
