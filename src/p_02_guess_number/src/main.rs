use std::cmp::Ordering;
use std::io; //引入io标准库

use rand::Rng;

fn main() {
    // let x = 5;
    // let y = 10;
    // //使用占位符打印输出
    // println!("x = {x} and y + 2= {}", y + 2);
    println!("Guess the number!");
    //创建随机数
    let secret_number = rand::rng().random_range(1..=100);

    //循环
    loop {
        println!("Please input your guess.");

        //使用let定义变量
        //在rust中 定义了变量就默认不可变 使用mut将其更改为可变变量
        let mut guess = String::new();

        //用于接收用户输入
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //转换数据类型
        //rust中可以使用同名的变量，更改变量类型和内容
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("gun gun gun!");
                continue;
            }
        };

        println!("Your guessed: {guess}");

        //使用cmp处理比较，这里使用的是一种匹配模式
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("Yes");
                break;
            }
            Ordering::Greater => println!("big"),
            Ordering::Less => println!("small"),
        }
    }

    println!("SecretNumber --> {secret_number}");
}
