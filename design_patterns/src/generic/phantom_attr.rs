struct Dog<Breed> {
    name: String,
    breed: PhantomData<Breed>,
}

struct Labrador {}
struct Retriever {}
struct Poodle {}
struct Dachshund {}

impl Dog<Labrador> {
    fn breed_name(&self) -> &str {
        "labrador"
    }
}

impl Dog<Retriever> {
    fn breed_name(&self) -> &str {
        "retriever"
    }
}

impl Dog<Poodle> {
    fn breed_name(&self) -> &str {
        "poodle"
    }
}

impl Dog<Dachshund> {
    fn breed_name(&self) -> &str {
        "dachshund"
    }
}

