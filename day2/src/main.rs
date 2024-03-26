fn main() {
    let result = add_num(2, 4);
    println!("Hello, world! {}", result);
    // ctrl + shift + enter = 末尾加分号
    let mut user_name = "王思聪";
    user_name = "王思聪";
    // 打印出名字
    println!("user_name: {}", user_name);
    // string split to collect
    let text = String::from("apple,banana,cherry");
    let fruits: Vec<&str> = text.split("banana").collect();
    println!("{:?}", fruits);
    // vec
    let vec_name = vec!["apple", "banana"];
    println!("first is:{}", vec_name[0]);
}


fn add_num(a: i32, b: i32) -> i32 {
    println!("加减法");
    return a + b;
}