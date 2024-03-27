use regex::Regex;

fn main() {
    println!("Hello, world!");
    let re = Regex::new(r#"\"双引号"#).unwrap();
    let text = "这是一个包含\"双引号\"的字符串";
    let replaced_text = re.replace_all(text, "'");
    println!("{}", replaced_text);
}
