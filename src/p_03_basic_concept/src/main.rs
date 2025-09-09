use num_rational::Rational64;
use num_traits::ToPrimitive;
fn main() {
    // let x = 5;
    // println!("The value of x is : {x}");
    // x = 6;  这里赋值会引入错误，变量不能更改
    //变量的定义
    var_define();
    //常量的定义
    var_define_const();
    //遮蔽
    var_define_mask();
    //数据类型
    data_type();
    //元组
    tuple_define();
    //数组
    array_define();
    //函数
    println!("{}-{}", five(100), five(1));
    //控制流-if
    control_flow_if();
    //控制流 - else if
    control_flow_else_if();

    //loop
    control_loop();

    //control_loop_result
    control_loop_result();

    //
    control_loop_tag();
}

/**
 * 定义常量
 */
fn var_define() {
    let mut x = 5;
    println!("The value of x is : {x}");
    x = 6;
    println!("The value of x is : {x}");
}

/**
 * 定义常量
 */
fn var_define_const() {
    const USER_DEFAULT_ID: u32 = 10000;
    println!("The value of USER_DEFAULT_ID is : {USER_DEFAULT_ID}");
}

/**
 * 遮蔽
 */
fn var_define_mask() {
    let var_1 = 10;
    {
        let var_1: u32 = 100;
        println!("The value of var_1 in the inner scope is : {var_1}");
    }
    println!("The value of var_1 is : {var_1}");
}

/**
 * 数据类型
 */
fn data_type() {
    let temp: u32 = 100;
    let temp_num: u32 = "123".parse().expect("转换失败！");

    //浮点类型
    let f = 2.0; //f64

    let d: f32 = 3.0; //f32

    //数值运算

    let sum = 5 + 10;
    let difference = 95.6 - 4.1;

    //rust中没有实现整数 * 浮点数这样的组合，因为rust是强类型语言，要求运算双方的类型完全一致
    //如果要实现整数 * 浮点数 需要将一方的数据进行转换，为了保证精度，这里选择向浮点类型转换
    //这里还会遇到另一个常见的问题，那就是精度丢失。 需要使用Rational64 进行计算
    let product = (4 as f64) * 3.14;

    // 创建分数 10/3
    let ratio = Rational64::new(10, 3);
    println!("10 / 3 = {}", ratio); // 输出：10/3（精确表示）

    let as_float = ratio.to_f64().unwrap_or(0.0);

    println!("转为浮点数：{}", as_float); // 输出：3.3333333333333335

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // 结果为 -1

    // remainder
    let remainder = 43 % 5;
}

/**元组定义 */
fn tuple_define() {
    //使用可循啊的类型注解声明一个元组
    let tup: (i32, f64, u8) = (500, 0.2, 1);

    //使用模式匹配解构元组
    let (x, y, z) = tup;
}

/**
 * 数组定义
 */
fn array_define() {
    let array_a = [1, 2, 3, 4, 5];
}

/**
 * 声明一个带返回值，带参数的函数
 */
fn five(number: i32) -> i32 {
    let s = { 100 + 20 };

    2 + 3 + number + s
}

/**
 * 控制流 - if
 */
fn control_flow_if() {
    let num = 3;
    if num < 5 {
        println!(" num :  {} 小于 5", num);
    } else {
        println!(" num :  {} 大于 5", num);
    }
}

/**
 * 控制流 - else if
 */
fn control_flow_else_if() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn control_loop() {
    let mut count = 0;

    loop {
        if count > 3 {
            break;
        }
        count = count + 1;
        println!("control_loop count >>> {count}");
    }
}

/**
 * loop 带结果
 */
fn control_loop_result() {
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 5;
        }
    };
    println!("control_loop_result >> result >> {result}")
}

/**
 * 嵌套循环-Tag
 */
fn control_loop_tag() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
