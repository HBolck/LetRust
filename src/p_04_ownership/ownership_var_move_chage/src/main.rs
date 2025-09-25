fn main() {
    println!("==ownership_var_move_chage==");

    println!("Hello, world!");

    intager_var_change();
}

/**
 * 整形类型的变量和交换
 */
fn intager_var_change() {
    let x = 5;
    let y = x;

    println!("x => {x}");
    println!("y => {y}");
}

fn string_var_change(){
    let s1 = String::from("hello");
    let s2 = s1;

    println!("s1 => {s1}");
    println!("s2 => {s2}");

}
