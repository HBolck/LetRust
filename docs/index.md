---
# 头部信息（Jekyll 专用，称为 Front Matter）
layout: default # 指定使用 _layouts/default.html 模板
title: 主页标题 # 页面标题（会替换模板中的 {{ page.title }}）
---

## 入门

Rust 程序设计语言的本质实际在于 赋能（empowerment）：无论你现在编写的是何种代码，Rust 能让你在更为广泛的编程领域走得更远，写出自信。（这一点并不显而易见）

举例来说，那些“系统层面”的工作涉及内存管理、数据表示和并发等底层细节。从传统角度来看，这是一个神秘的编程领域，只为浸润多年的极少数人所触及，也只有他们能避开那些臭名昭著的陷阱。即使谨慎的实践者，亦唯恐代码出现漏洞、崩溃或损坏。

Rust 破除了这些障碍：它消除了旧的陷阱，并提供了伴你一路同行的友好、精良的工具。想要 “深入” 底层控制的程序员可以使用 Rust，无需时刻担心出现崩溃或安全漏洞，也无需因为工具链不靠谱而被迫去了解其中的细节。更妙的是，语言设计本身会自然而然地引导你编写出可靠的代码，并且运行速度和内存使用上都十分高效。

已经在从事编写底层代码的程序员可以使用 Rust 来提升信心。例如，在 Rust 中引入并行是相对低风险的操作，因为编译器会替你找到经典的错误。同时你可以自信地采取更加激进的优化，而不会意外引入崩溃或漏洞。

但 Rust 并不局限于底层系统编程。它表达力强、写起来舒适，让人能够轻松地编写出命令行应用、网络服务器等各种类型的代码——在本书中就有这两者的简单示例。使用 Rust 能让你把在一个领域中学习的技能延伸到另一个领域：你可以通过编写网页应用来学习 Rust，接着将同样的技能应用到你的 Raspberry Pi（树莓派）上。

本书全面介绍了 Rust 为用户赋予的能力。其内容平易近人，致力于帮助你提升 Rust 的知识，并且提升你作为程序员整体的理解与自信。欢迎你加入 Rust 社区，让我们准备深入学习 Rust 吧！

—— Nicholas Matsakis 和 Aaron Turon

👆 这里是《Rust 程序设计语言》中前言的部分，这个项目将按照这本书的示例、内容进行编码，最终目的是学会 Rust，会用 Rust

这本书的原作者是 Steve Klabnik 和 Carol Nichols ，由 Rust 社区补充完整

[《Rust 程序设计语言 中文版》](https://kaisery.github.io/trpl-zh-cn/foreword.html) 由中文社区翻译

<div style="border: 1px solid #eee; padding: 15px; border-radius: 5px; background: #f9f9f9;">
  <h3 style="margin-top: 0;">目录</h3>
  <ol style="padding-left: 20px;">
    <li><a href="index.md">一、入门</a>
      <ul>
        <li><a href="./src/01/1.1_install.md">1.1 安装</a></li>
        <li><a href="./src/01/1.2_create.md">1.2 项目创建</a></li>
      </ul>
    </li>
    <li><a href="#2-核心概念">二、核心概念</a></li>
    <!-- 其他章节 -->
  </ol>
</div>
