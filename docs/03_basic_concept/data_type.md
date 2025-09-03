<link rel="stylesheet" href="../style/main.css">

## 数据类型

在 Rust 中，每个值都有对应的 `数据类型` ，通过这个部分告诉 Rust 在代码中，数据将以什么方式存放在内存中。

这里有两种类型

- 标量 (scalar)
- 复合 (compound)

```
Rust 是静态型的语言，也就是说在编译的过程中，就必须知道所有的变量类型。根据值和使用方式，编译器通常可以猜测出适合的类型。
```

声明一个指定数据类型可以按照下面的方式进行

```rust
let temp : u32 = 100;
```

如果想要声明一个已有类型转换到目标类型的操作，可以这样做。

这是将一个 字面量为 123 的字符串转换到无符号整型的变量 `temp_num` 中去。如果不添加 `: u32` 这样的类型声明，那么编译器就不知道需要转换成什么类型，这个时候将 <span class="error_text">报错</span>。

```rust
let temp_num: u32 = "123".parse().expect("转换失败！");
```

### 标量类型

标量(scalar) 类型代表一个单独的值。Rust 有四种基本的标量类型：整型、浮点型、布尔类型和字符类型。

#### 整型

整型是没有小数部分的数字。之前使用过`u32`整数类型。它关联的值应该是一个占据 32 比特的无符号整数（有符号是 `i` 开头 无符号是 `u` 开头）

下面的表格中展示了 Rust 内建的整数类型：

| 长度     | 有符号  | 无符号   |
| -------- | ------- | -------- |
| 8-bit    | `i8`    | `u8`     |
| 16-bit   | `i16`   | `u16`    |
| 32-bit   | `i32`   | `u32 `   |
| 64-bit   | `i64`   | `u64`    |
| 128-bit  | `i128`  | `u128  ` |
| 架构相关 | `isize` | `usize`  |

每一个变体都可以是有符号或无符号的，并有一个明确的大小。`有符号`和`无符号`的区别是数字是否可以为负数。有符号数通过 [二进制补码形式（two’s complement representation） ](https://en.wikipedia.org/wiki/Two%27s_complement)

另外，`isize` 和 `usize` 类型依赖运行程序的计算机架构：64 位架构上它们是 64 位的，32 位架构上它们是 32 位的。

**核心区别**
|类型 | 含义 | 取值范围 | 典型用途 |
|----|----|----|----|
|`isize` | 有符号平台相关整数 | 32 位系统：-2³¹ ~ 2³¹-1 | 表示有符号的内存偏移量|
|||`64 位系统：-2⁶³ ~ 2⁶³-1 `|（如指针运算中的正负偏移）|
|`usize` | 无符号平台相关整数 | 32 位系统：0 ~ 2³²-1 | 表示内存大小、集合索引
|||`64 位系统：0 ~ 2⁶⁴-1 `| （如数组长度、向量索引）|

**关键特性**

1.与平台位数一致

- 在 32 位操作系统 / 架构上，isize 和 usize 都是 32 位（4 字节）
- 在 64 位操作系统 / 架构上，两者都是 64 位（8 字节）

这意味着它们能精确表示当前平台的内存地址范围（例如 64 位系统可寻址更大的内存空间）。

2.usize 与集合索引

Rust 中所有集合类型（如 Vec、[T; N]、HashMap）的索引类型都是 usize，例如：

```rust
let v = vec![1, 2, 3];
let index: usize = 0;  // 必须用 usize 作为索引
println!("{}", v[index]);  // 正确

let bad_index: i32 = 0;
// println!("{}", v[bad_index]);  // 错误：索引必须是 usize
```

3.isize 与内存偏移

常用于需要正负偏移的场景（如指针运算），例如：

```rust
let arr = [10, 20, 30, 40];
let ptr = arr.as_ptr();  // 获取数组首地址（*const i32）

// 计算偏移 +1 的地址（指向 20）
let offset_ptr = unsafe { ptr.offset(1) };
// 计算偏移 -1 的地址（指向 10，需确保在合法范围内）
let neg_offset_ptr = unsafe { ptr.offset(-0) };
```

这里 offset 方法的参数类型是 isize，允许正负值表示向前 / 向后偏移。

**使用建议**

- 优先用 usize 的场景：
  - 集合索引（数组、向量、切片等）
  - 表示长度、容量（如 `Vec::len()` 返回 `usize`）
  - 内存大小相关计算（如分配内存的字节数）
- 优先用 isize 的场景：
  - 需要正负偏移的指针运算
  - 表示相对位置（如两个地址的差值）
- 避免滥用：
  - 不建议用于普通数值计算（优先用 `i32`、`u32` 等固定长度类型，确保跨平台一致性）
  - 注意 usize 无符号的特性，避免负数赋值或下溢（如 `usize::MAX - 1` 需谨慎处理）

示例代码

```rust
fn main() {
    // 打印当前平台下的类型大小（字节）
    println!("isize 大小: {} 字节", std::mem::size_of::<isize>());  // 4 或 8
    println!("usize 大小: {} 字节", std::mem::size_of::<usize>());  // 4 或 8

    // 集合索引必须用 usize
    let nums = [1, 2, 3, 4];
    let idx: usize = 2;
    println!("nums[{}] = {}", idx, nums[idx]);  // 输出 nums[2] = 3

    // 指针偏移使用 isize
    let ptr = nums.as_ptr();
    let offset: isize = 1;
    let shifted_ptr = unsafe { ptr.offset(offset) };  // 指向 nums[1]
    println!("偏移后的值: {}", unsafe { *shifted_ptr });  // 输出 2
}
```

##### 整型字面量

在 Rust 中，存在整型的字面量，这些字面量被允许在一些特定的场景使用。

| 数字字面量                   | 例子          |
| ---------------------------- | ------------- |
| Decimal (十进制)             | `98_222`      |
| Hex(十六进制)                | `0xff`        |
| Octal(八进制)                | `0o77`        |
| Binary (二进制)              | `0b1111_0000` |
| Byte (单字节字符)(仅限于 u8) | `b'A'`        |

如果不确定使用哪种类型，那么一般使用`i32`、`isize`、`usize`作为一些基础类型使用。

#### 浮点型

Rust 存在两种原生的 `浮点数` 类型。`f32` `f64`

```rust
//浮点类型
let f = 2.0; //f64
let d: f32 = 3.0; //f32
```

##### 数值运算

```rust
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // 结果为 -1

    // remainder
    let remainder = 43 % 5;
}
```

#### 布尔类型

Rust 提供了一种布尔类型，值同样为 `true` `false`。

```rust
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```

#### 字符类型

Rust 的 `char` 类型是语言中最原始的字母类型。

```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
```

### 复合类型

复合类型(Compound types) 可以将多个值合成一个类型。Rust 有两个原生的复合类型:元组(tuple)和数组(array)

#### 元组类型

元组是一个将多个不同类型的值组合进一个复合类型的主要方式。元组长度固定：一旦声明，其长度不会增大或缩小。

在 Rust 中，使用包含在圆括号中的逗号分隔的值来创建和表示一个元组。

元组的每一个位置都有一个类型，而且这些不同值得类型也不必是相同的。下面的例子中使用了可选的类型注解：

```rust
fn main(){
  let tup: (i32, f64, u8) = (500, 0.2, 1);
}
```

`tup` 变量绑定到整个元组上，因为元组是一个单独的复合元素。为了从元组中获取单个值，可以使用模式匹配来解构元组值。

```rust
  //使用模式匹配解构元组
    let (x, y, z) = tup;

```
这里简单的对上面这行代码进行一个说明。

这里使用了 `let` 和一个模式将 `tup` 分成了三个不同的变量，`x` `y` `z` 。这个过程叫做 `解构(destructuring)` 

按照编程的习惯，也可以使用下面的方式进行访问元素。

```rust

fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}

```

不带有任何值的元组有个特殊的名称，叫做 **单元(unit)**。这种值以及对应的类型都写作 `()` 表示空值或空的返回类型。如果表达式不返回任何其他值，则会隐式返回单元值。


#### 数组类型

另一个包含多个值的方式是 `数组(array)`。与元组不同的地方在于，数组中的每个元素类型必须一致，Rust中的数组与一些其他语言中的数组不同，在Rust中，数组的长度是固定且不可变的。

下面是一个示例：

```rust
fn main(){
   //构建一个长度为5 i32类型的数组
   let array_a = [1, 2, 3, 4, 5];
}
```

如果有个场景需要固定数量的元素数量的时候，数组是一个不错的选择。

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

```

数组的声明方式有很多种，下面的几种方式都能够快速的创建一个数组。

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];

let a = [3; 5]; //let a = [3, 3, 3, 3, 3];

```

值得说明的是，数组是可以被分配在栈上的固定已知大小的内存块。可以使用索引来访问数组，索引的其实下标是 `0`。

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

在使用下标访问数组的时候，需要注意数组越界的问题。这个问题在`check`过程中是检查不出来的，只有`运行时`才可以被抛出。

`rust是静态语言，没有运行时，这里又提到了运行时，这个运行时又是什么呢？ 这里暂时不纠结，只要知道rust可以在运行检查中抛出异常就可以了。`

```rust
use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
```

上面的代码会输出下面的异常内容,在后面的内容中，将使用 `恐慌`和`避免恐慌`的概念来处理这种异常的输出，进而保证程序的正常运行

```
thread 'main' panicked at src/main.rs:19:19:
index out of bounds: the len is 5 but the index is 10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```