fn main() {
    let new = Empty {};
    let test: Test = Test::first;
    println!("Hello, world!");
}



pub struct Empty {}

pub enum Test {
    first,
    second,
    third
}
