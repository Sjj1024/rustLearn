fn main() {
    let user_name = "王思聪";
    let user_age = 18;
    // let user_age = "年龄";
    let mut user_home = "洛阳";
    let user_home = "上海";
    println!("用户名和年龄是: {}, age:{}，home: {}", user_name, user_age, user_home);
    // 定义一个元组，然后遍历
    let citys = ("洛阳", "上海", "北京");
    for_tuble(citys);
    // 定义数组并遍历
    let users = ["王健林", "王思聪", "王宝强", "马云", "马化腾"];
    for_list(users);
    // 三元表达式:
    let user_res = if user_age > 19 { 1 } else { -1 };
    println!("三元表达式结果是: {}", user_res);
    // while循环
    while_func(4);
    // loop
    loop_fn();
    // 基本数据类型
    let user_type = "新手玩家";
    let mut copy_user = user_type;
    // copy_user = "老司机";
    println!("user type: {}, cope_user: {}", user_type, copy_user);
    // 字符串类型
    use_string();
    // 切片
    struct_fn();
}


// 加减乘除
fn add_num(a: i32, b: i32) -> i32 {
    return a + b;
}

// 乘法
fn sub_num(a: i32, b: i32) -> u32 {
    return (a * b) as u32;
}

// 除法
fn dub_num(a: f64, b: f64) -> f64 {
    return a / b;
}

// 遍历元组类型
fn for_tuble(tub: (&str, &str, &str)) {
    println!("for_tuble: {}", tub.0);
    println!("for_tuble: {}", tub.1);
    println!("for_tuble: {}", tub.2);
}


// 遍历数组类型
fn for_list(users: [&str; 5]) {
    for i in users {
        println!("for user is {}", i);
    }
}


//  while
fn while_func(max: i32) {
    let mut num = 0;
    while num < max {
        println!("while fn num is : {}", num);
        num += 1;
    }
}

// loop 循环
fn loop_fn() {
    let mut start = 0;
    loop {
        if start > 10 {
            break;
        }
        println!("loop current at index: {}", start);
        start += 1;
    }
}


// string
fn use_string() {
    // 将&str转为String类型
    let mut info = String::from("asdljapiowernpsn");
    info.push_str("王思聪");
    println!("使用字符串: {}", info);
    // 切片
    let splice1 = &info[0..4];
    // 将String转为&str
    let splice2 = &info[..];
    println!("切片类型splice1:{}, string:{}", splice1, splice2);
    // &str类型操作
}

// 结构体
fn struct_fn() {
    #[derive(Debug)]
    struct User {
        name: String,
        password: String,
        age: i32,
        hobys: [i32; 5],
    }

    let user = User {
        name: String::from("王思聪"),
        password: String::from("2esasdf"),
        age: 18,
        hobys: [1, 24, 56, 7, 8],
    };

    println!("user info is: {:?}", user);
}