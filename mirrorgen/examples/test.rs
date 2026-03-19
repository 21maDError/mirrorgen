use mirrorgen::Mirror;

#[derive(Mirror)]
#[mirror(name = "TestDTO", omit = [a, b], rename = [a => x, b => y])]
//#[mirror(name = "another", rename = [b => y])]
struct Test {
    a: i32,
    b: i32,
}

fn main() {
}