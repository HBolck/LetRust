fn main() {
    // let x = 5;
    // println!("The value of x is : {x}");
    // x = 6;  这里赋值会引入错误，变量不能更改

    //变量的定义
    let mut x = 5;
    println!("The value of x is : {x}");
    x = 6;
    println!("The value of x is : {x}");

    //常量的定义
    const USER_DEFAULT_ID: u32 = 10000;
    println!("The value of USER_DEFAULT_ID is : {USER_DEFAULT_ID}");

    //遮蔽
    let var_1 = 10;
    {
        let var_1: u32 = 100;
        println!("The value of var_1 in the inner scope is : {var_1}");
    }
    println!("The value of var_1 is : {var_1}");
}
