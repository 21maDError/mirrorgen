use mirrorgen::Mirror;

#[derive(Mirror, Debug)]
#[mirror(name = "TestDTO", omit = [a], rename = [b => y])]
#[mirror(name = "another", rename = [b => y])]
struct Test {
    a: i32,
    b: i32,
}

fn main() {
    let test = Test { a: 67, b: 90 };
    let test_dto = TestDTO { y: 90 };

    dbg!(test);
    dbg!(test_dto);
}
