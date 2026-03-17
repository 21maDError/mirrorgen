use mirrorgen::Mirror;

#[derive(Mirror)]
#[mirror(name = "Test", omit = [a, b])]
struct Test {
    a: i32,
    b: i32,
}

fn main() {
    //let test = Test { a: 1, b: 2 };
    Test::hello();
}
