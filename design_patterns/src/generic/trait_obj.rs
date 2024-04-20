trait MyTrait {
    fn trait_hello(&self);
    fn as_any(&self) -> &dyn Any;
}

impl MyTrait for MyStruct1 {
    fn trait_hello(&self) {
        self.struct_hello();
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl MyStruct1 {
    fn struct_hello(&self) {
        println!("Hello, world! from MyStruct1");
    }
}

impl MyTrait for MyStruct2 {
    fn trait_hello(&self) {
        self.struct_hello();
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl MyStruct2 {
    fn struct_hello(&self) {
        println!("Hello, world! from MyStruct2");
    }
}


/*
 *  println!("With a downcast:");
    v.iter().for_each(|i| {
        if let Some(obj) = i.as_any().downcast_ref::<MyStruct1>() {
            obj.struct_hello();
        }
        if let Some(obj) = i.as_any().downcast_ref::<MyStruct2>() {
            obj.struct_hello();
        }
    });
 *
 * */
