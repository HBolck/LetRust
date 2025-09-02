<link rel="stylesheet" href="../style/main.css">

## 常见的概念

### 变量与可变性

#### 变量

在 Rust 中的变量默认是不可变的，这个看起来可能很奇怪，其他的语言都能够变更内容，在`C#`中可以添加`readonly`这样的关键字来修饰这个变量是只读的，类似的还有很多。

但是这种风格成为了 Rust 的一种优势，可以让编码变得安全和简单。虽然默认是不可更改的，但是也可以通过关键字 `mut`来实现可改的特性。

在 Rust 中 可以通过 `let` 关键字来创建变量。

```
Rust中的变量和函数的命名风格推荐使用蛇形命名法（snake_case），即全部小写字母，单词之间用下划线 _ 连接。
```

在下面的代码中可以看到更改不可变变量引入的错误。

文件名：`src/p_03_basic_concept/src/main.rs`

```rust
fn main() {
    let x = 5;
    println!("The value of x is : {x}");
    x = 6;
}
```

```rust
warning: value assigned to `x` is never read
 --> src\main.rs:4:5
  |
4 |     x = 6;
  |     ^
  |
  = help: maybe it is overwritten before being read?
  = note: `#[warn(unused_assignments)]` on by default

error[E0384]: cannot assign twice to immutable variable `x`
 --> src\main.rs:4:5
  |
2 |     let x = 5;
  |         - first assignment to `x`
3 |     println!("The value of x is : {x}");
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut x = 5;
  |         +++

For more information about this error, try `rustc --explain E0384`.
warning: `p_03_basic_concept` (bin "p_03_basic_concept") generated 1 warning
error: could not compile `p_03_basic_concept` (bin "p_03_basic_concept") due to 1 previous error; 1 warning emitted
```

为了解决这个问题，可以使用刚才提到的关键字 `mut`

代码如下：

```rust
let mut x = 5;
println!("The value of x is : {x}");
x = 6;
println!("The value of x is : {x}");
```

输出内容：

```
The value of x is : 5
The value of x is : 6
```

通过上面的示例，很明显的可以看到通过 `mut` 修饰的变量从 5 变成了 6

#### 常量

说到常量，第一个想到的就是不可变的变量。在 Rust 中，通常使用`const`修饰，常量不光默认不可变，并且不允许使用 `mut` 进行修饰。

下面是一个常见的例子

```rust
const USER_DEFAULT_ID: u32 = 10000;
```

这里面出现了一个没有提到的内容 `: u32` 。这个奇怪的家伙是数据类型，因为 Rust 通过`const`定义常量数据，但是并不能像 `let` 那样可以推断出这个常量的类型，所以这里需要显式的声明一下这个常量属于什么数据类型，关于数据类型这个概念，将在未来详细说明。

#### 遮蔽

这个概念在 [第二章节中的示例代码中有体现](../../src/p_02_guess_number/src/main.rs)。

具体的操作就是，定义一个与之前变量同名的新变量，开发者们常常称第一个变量被第二个变量 `遮蔽` 。这意味着，当使用这个变量名的时候，编译器将看到第二个变量，也将取出第二个变量的值进行运算，知道第二个变量的作用域（是一对花括号，后面会将）结束。

那第一个变量呢？当然是被遮盖掉了！（这里引入了`所有权`的概念，当遮蔽发生时，第一个变量值的所有权并没有被移动到第二个变量。第一个变量只是被“遮盖”而无法在当前作用域内再被直接访问。它会在当前作用域结束的位置被正常丢弃（Drop）。如果它的类型实现了 Drop Trait，它的析构函数也会在那个时候被调用 这里不用太纠结 `所有权` 知道就行！）

这段代码可以很清楚的理解遮蔽的概念

```rust
let var_1 = 10;
{
    let var_1: u32 = 100;
    println!("The value of var_1 in the inner scope is : {var_1}");
}
println!("The value of var_1 is : {var_1}");
```

输出：

```
The value of var_1 in the inner scope is : 100
The value of var_1 is : 10
```

上面的代码创建了一个 var_1 的变量，赋值为 10。 代码进入一个新的作用域 `{}` 在新的作用域中，创建了一个同名的变量 var_1 赋值为 100。
这个时候，var_1 被遮盖了，在这个作用域中，var_1 的值是 100，当离开这个作用域的时候，第一个变量的遮盖效果被去掉了，所以数值恢复到了 10

### 数据类型

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

#### 标量类型

标量(scalar) 类型代表一个单独的值。Rust 有四种基本的标量类型：整型、浮点型、布尔类型和字符类型。

##### 整型

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

###### 整型字面量

在 Rust 中，存在整型的字面量，这些字面量被允许在一些特定的场景使用。

| 数字字面量                   | 例子          |
| ---------------------------- | ------------- |
| Decimal (十进制)             | `98_222`      |
| Hex(十六进制)                | `0xff`        |
| Octal(八进制)                | `0o77`        |
| Binary (二进制)              | `0b1111_0000` |
| Byte (单字节字符)(仅限于 u8) | `b'A'`        |

如果不确定使用哪种类型，那么一般使用`i32`、`isize`、`usize`作为一些基础类型使用。

##### 浮点型

Rust 存在两种原生的 `浮点数` 类型。`f32` `f64`

```rust
//浮点类型
let f = 2.0; //f64
let d: f32 = 3.0; //f32
```

###### 数值运算

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

##### 布尔类型

Rust 提供了一种布尔类型，值同样为 `true` `false`。

```rust
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```

##### 字符类型

Rust 的 `char` 类型是语言中最原始的字母类型。

```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
```

#### 复合类型

复合类型(Compound types) 可以将多个值合成一个类型。Rust 有两个原生的复合类型:元组(tuple)和数组(array)

##### 元组类型

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