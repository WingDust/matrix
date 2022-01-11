/*\ ##
|*| - [paste](https://github.com/dtolnay/paste)
|*| - [syn](https://github.com/dtolnay/syn)
|*| - [docopt.rs](https://github.com/docopt/docopt.rs)
|*| - [slag](https://github.com/mystor/slag)
\*/

/*\ ## 设计语言
|*| - [QinScript](https://github.com/Yaser-wyx/QinScript)
|*| - [圣杯与银弹 · 没用的设计模式](https://draveness.me/holy-grail-design-pattern/)
|*| - [梦寐以求的编程语言](https://www.kancloud.cn/imxieke/hacker-and-painter/107333)
|*| - [自制编译器+自制脚本语言+自制编程语言 三书比较？](https://www.zhihu.com/question/27686032)
|*| - [《自制编程语言》学习笔记](https://github.com/yifengyou/sparrow)
|*| - [如何使用 JavaScript 实现一门编程语言(5) ](https://www.kancloud.cn/xiaoyulive/system/606567)
|*| - [记录一下如何实现一门编程语言~(1)](https://juejin.cn/post/6936133723481964575)
|*| - [90分钟实现一门编程语言——极简解释器教程](https://www.cnblogs.com/figure9/p/3620079.html)
|*| - [分享 如何制作编程语言-Bean 语言的设计与实现](https://ruby-china.org/topics/39711)
|*| - [我写了一门编程语言，你也可以！](https://www.jianshu.com/p/2163aa15d19e)
|*| - [13 | LLVM：如何将自定义的语言编译到 WebAssembly？](https://time.geekbang.org/column/article/293272)
|*| - [小.wasm尺寸](http://llever.com/rustwasm-book/print.html)
|*| - [wasm的编译](https://github.com/XChainLab/documentation/blob/master/VM/evm-ewasm/wasm%E7%9A%84%E7%BC%96%E8%AF%91.md)
\*/

use std::fs;

#[allow(dead_code)]
const KEYWORDS: [&str; 2] = ["fn", "if", "and", "or", "not"];

fn main() {
    parse();
}

#[allow(dead_code)]
fn parse() {
    let content = fs::read_to_string("./src/a.txt").expect("");
    println!("{}", content);
}

// [1,2,3].foo.app
// foo([1,2,3]).app()

// name.foo.app
// foo(name).app()

// "qwe".foo.app.
// foo("qwe").app()
#[allow(dead_code)]
#[allow(unused_macros)]
macro_rules! dot {
    ($var:literal$(.$fn:ident)+) => {
        println!("wq");
    };
    // [How do I generalize a Rust macro over different types of functions?](https://stackoverflow.com/questions/35590450/how-do-i-generalize-a-rust-macro-over-different-types-of-functions)
    ($var:ident$(.$fn:ident)+) => {};
}
