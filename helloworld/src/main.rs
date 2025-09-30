// using this project to just go crazy with it yo


struct Foo {
    id: i32,
    name: String,
}

fn main() {
    println!("Hello, world!");

    let dog = Foo {
        id: 0,
        name: String::from("Some new foo"),
    };

    println!("{} - {}", dog.id, dog.name)



}
