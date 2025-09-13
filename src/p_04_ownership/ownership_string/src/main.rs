fn main() {
    println!("==ownership_string==");

    let mut s = String::from("hello"); //创建一个可以变更内容的字符串类型的变量

    s.push_str(", world.");//追加字符串

    println!("{s}");//打印这个变量的内容
}
