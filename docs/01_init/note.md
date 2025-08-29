## 入门
```
Rust 程序设计语言的本质实际在于 赋能（empowerment）：无论你现在编写的是何种代码，Rust 能让你在更为广泛的编程领域走得更远，写出自信。（这一点并不显而易见）

举例来说，那些“系统层面”的工作涉及内存管理、数据表示和并发等底层细节。从传统角度来看，这是一个神秘的编程领域，只为浸润多年的极少数人所触及，也只有他们能避开那些臭名昭著的陷阱。即使谨慎的实践者，亦唯恐代码出现漏洞、崩溃或损坏。

Rust 破除了这些障碍：它消除了旧的陷阱，并提供了伴你一路同行的友好、精良的工具。想要 “深入” 底层控制的程序员可以使用 Rust，无需时刻担心出现崩溃或安全漏洞，也无需因为工具链不靠谱而被迫去了解其中的细节。更妙的是，语言设计本身会自然而然地引导你编写出可靠的代码，并且运行速度和内存使用上都十分高效。

已经在从事编写底层代码的程序员可以使用 Rust 来提升信心。例如，在 Rust 中引入并行是相对低风险的操作，因为编译器会替你找到经典的错误。同时你可以自信地采取更加激进的优化，而不会意外引入崩溃或漏洞。

但 Rust 并不局限于底层系统编程。它表达力强、写起来舒适，让人能够轻松地编写出命令行应用、网络服务器等各种类型的代码——在本书中就有这两者的简单示例。使用 Rust 能让你把在一个领域中学习的技能延伸到另一个领域：你可以通过编写网页应用来学习 Rust，接着将同样的技能应用到你的 Raspberry Pi（树莓派）上。

本书全面介绍了 Rust 为用户赋予的能力。其内容平易近人，致力于帮助你提升 Rust 的知识，并且提升你作为程序员整体的理解与自信。欢迎你加入 Rust 社区，让我们准备深入学习 Rust 吧！

—— Nicholas Matsakis 和 Aaron Turon
```

👆 这里是《Rust 程序设计语言》中前言的部分，这个项目将按照这本书的示例、内容进行编码，最终目的是学会Rust，会用Rust

这本书的原作者是 Steve Klabnik 和 Carol Nichols ，由Rust社区补充完整

[《Rust 程序设计语言 中文版》](https://kaisery.github.io/trpl-zh-cn/foreword.html)  由中文社区翻译


### 安装

[windows安装Rust](https://www.rust-lang.org/zh-CN/tools/install)


#### Linux或macOS 安装

```shell
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```
这个命令将安装最新稳定版本的Rust，如果安装成功，将出现下面的内容
```
Rust is installed now. Great!
```
另外还需要安装一个 *链接器* 安装这个家伙的作用是将编译的输出链接成一个文件。如果遇到了链接器错误的时候，尝试用下面的命令安装

macOS

```shell
$ xcode-select --install
```

Linux用户通常需要根据发行版的文档安装 GCC 或者 Clang 

如果是 Ubuntu 用户需要安装 `build-essential` 包。

上面提到的链接器，如果是window用户，则不需要关心这些问题，因为官方提供的安装器在默认安装的时候会安装所有需要的环境和工具

### 更新

使用下面的命令进行更新rust

```shell
rustup update
```

### 卸载

```shell
rustup self uninstall
```

### 开发工具

这里个人推荐使用Vscode

#### 推荐安装的插件

|序号|插件名称|
|----|----|
|1|rust|
|2|Rust Syntax|
|3|Dependi|
|4|rust-analyzer|

### 创建项目

#### 手动创建项目

可以使用Vscode打开一个空目录，创建一个后缀名称是.rs的文件，打开后在里面输入下面的内容：

```rust
fn main() {
    println!("Hello, world!");
}
```

#### 使用Cargo创建项目

Cargo 是 Rust 的构建系统和包管理器。大多数 Rustacean 们使用 Cargo 来管理他们的 Rust 项目，因为它可以为你处理很多任务，比如构建代码、下载依赖库并编译这些库。（我们把代码所需要的库叫做 依赖（dependencies））。

最简单的 Rust 程序，比如我们刚刚编写的，没有任何依赖。如果使用 Cargo 来构建 “Hello, world!” 项目，将只会用到 Cargo 构建代码的那部分功能。在编写更复杂的 Rust 程序时，你将添加依赖项，如果使用 Cargo 启动项目，则添加依赖项将更加容易。

由于绝大多数 Rust 项目使用 Cargo，本书接下来的部分假设你也使用 Cargo。如果使用 “安装” 部分介绍的官方安装包的话，则自带了 Cargo。如果通过其他方式安装的话，可以在终端输入如下命令检查是否安装了 Cargo：

```shell
$ cargo --version
```
如果你看到了版本号，说明已安装！如果看到类似 command not found 的错误，你应该查看相应安装文档以确定如何单独安装 Cargo。
```
cargo 1.89.0 (c24e10642 2025-06-23)
```

使用下面的命令创建一个项目

```shell
cargo new hello_cargo
cd hello_cargo
```

控制台会输出下面的内容
```
Creating binary (application) `hello_cargo` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```

通过Cargo创建的项目，会默认带出一个git环境，如果不需要创建这个git环境可以使用下面的这个命令创建项目。Cargo还有一个特性，创建项目的时候会检测父级目录中是否存在git环境，如果存在，则不会再创建新的git环境，这样会避免出现git嵌套的问题。

```shell
cargo new --vcs none hello_cargo
```

##### Cargo的其他命令

检查项目，这个通常是做语法检查
```shell
cargo check
```
编译项目，一般可以直接进行编译，编译前会进行check动作，出现语法错误会中断编译动作
```shell
cargo build
```
运行项目
```shell
cargo run
```


使用Cargo会引入一个新的文件 `Cargo.toml`

这个文件使用 TOML (Tom's Obvious, Minimal Language) 格式，这是 Cargo 配置文件的格式。

第一行，[package]，是一个片段 section 标题，表明下面的语句用来配置一个包。随着我们在这个文件增加更多的信息，还将增加其他 section。

接下来的三行设置了 Cargo 编译程序所需的配置：项目的名称、项目的版本以及要使用的 Rust 版本。附录 E 会介绍 edition 的值。

最后一行，[dependencies]，是罗列项目依赖的 section 的开始。在 Rust 中，代码包被称为 crates。这个项目并不需要其他的 crate，不过在第二章的第一个项目会用到依赖，那时会用得上这个 section。

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"

[dependencies]
```