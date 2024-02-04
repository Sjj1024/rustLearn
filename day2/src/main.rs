fn main() {
    let result = add_num(2, 4);
    println!("Hello, world! {}", result);
}


fn add_num(a: i32, b: i32) -> i32 {
    println!("加减法");
    return a + b;
}