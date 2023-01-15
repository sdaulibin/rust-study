## 格式化输出
### 打印操作由 `std::fmt` 里面所定义的一系列宏来处理，包括：
- `format!`：将格式化文本写到字符串。
- `print!`：与 `format!` 类似，但将文本输出到控制台（`io::stdout`）。
- `println!`: 与 `print!` 类似，但输出结果追加一个换行符。
- `eprint!`：与 `print!` 类似，但将文本输出到标准错误（`io::stderr`）。
- `eprintln!`：与 `eprint!` 类似，但输出结果追加一个换行符。

### `std::fmt` 包含多种 trait（特质）来控制文字显示，其中重要的两种 trait 的基本形式如下：
- `fmt::Debug`：使用 `{:?}` 标记。格式化文本以供调试使用。
- `fmt::Display`：使用 `{}` 标记。以更优雅和友好的风格来格式化文本。


### 所有的类型，若想用 `std::fmt` 的格式化打印，都要求实现至少一个可打印的 `traits`。仅有一些类型提供了自动实现，比如 `std`库中的类型。所有其他类型都必须手动实现。

### `fmt::Debug` 这个 `trait` 使这项工作变得相当简单。所有类型都能推导（`derive`，即自动创建）fmt::Debug 的实现。但是 fmt::Display 需要手动实现。

```rust
#![allow(unused)]
fn main() {
// 这个结构体不能使用 `fmt::Display` 或 `fmt::Debug` 来进行打印。
struct UnPrintable(i32);

// `derive` 属性会自动创建所需的实现，使这个 `struct` 能使用 `fmt::Debug` 打印。
#[derive(Debug)]
struct DebugPrintable(i32);
}
```

### `fmt::Debug` 通常看起来不太简洁，因此自定义输出的外观经常是更可取的。这需要通过手动实现 `fmt::Display` 来做到。`fmt::Display` 采用 `{}` 标记。实现方式看起来像这样：

```rust
#![allow(unused)]
fn main() {
    // （使用 `use`）导入 `fmt` 模块使 `fmt::Display` 可用
    use std::fmt;

    // 定义一个结构体，咱们会为它实现 `fmt::Display`。以下是个简单的元组结构体
    // `Structure`，包含一个 `i32` 元素。
    struct Structure(i32);

    // 为了使用 `{}` 标记，必须手动为类型实现 `fmt::Display` trait。
    impl fmt::Display for Structure {
        // 这个 trait 要求 `fmt` 使用与下面的函数完全一致的函数签名
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // 仅将 self 的第一个元素写入到给定的输出流 `f`。返回 `fmt:Result`，此
            // 结果表明操作成功或失败。注意 `write!` 的用法和 `println!` 很相似。
            write!(f, "{}", self.0)
        }
    }
}
```
### `fmt::Display` 的效果可能比 `fmt::Debug` 简洁，但对于 `std` 库来说，这就有一个问题。模棱两可的类型该如何显示呢？举个例子，假设标准库对所有的 `Vec<T>` 都实现了同一种输出样式，那么它应该是哪种样式？下面两种中的一种吗？

- `Vec<path>`：/:/etc:/home/username:/bin（使用 : 分割）
- `Vec<number>`：1,2,3（使用 , 分割）

> 我们没有这样做，因为没有一种合适的样式适用于所有类型，标准库也并不擅自规定一种样式。对于 `Vec<T>` 或其他任意泛型容器（`generic container`），`fmt::Display` 都没有实现。因此在这些泛型的情况下要用 `fmt::Debug`。
> 这并不是一个问题，因为对于任何非泛型的容器类型， `fmt::Display` 都能够实现。

```rust
use std::fmt; // 导入 `fmt`

// 带有两个数字的结构体。推导出 `Debug`，以便与 `Display` 的输出进行比较。
#[derive(Debug)]
struct MinMax(i64, i64);

// 实现 `MinMax` 的 `Display`。
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 使用 `self.number` 来表示各个数据。
        write!(f, "({}, {})", self.0, self.1)
    }
}

// 为了比较，定义一个含有具名字段的结构体。
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// 类似地对 `Point2D` 实现 `Display`
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 自定义格式，使得仅显示 `x` 和 `y` 的值。
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // 报错。`Debug` 和 `Display` 都被实现了，但 `{:b}` 需要 `fmt::Binary`
    // 得到实现。这语句不能运行。
    // println!("What does Point2D look like in binary: {:b}?", point);
}
```
### `fmt::Display` 被实现了，而 `fmt::Binary` 没有，因此 `fmt::Binary` 不能使用。 `std::fmt` 有很多这样的 `trait`，它们都要求有各自的实现。

## 测试实例：List

### 对一个结构体实现 `fmt::Display`，其中的元素需要一个接一个地处理到，这可能会很麻烦。问题在于每个 `write!` 都要生成一个 `fmt::Result`。正确的实现需要处理所有的 `Result`。`Rust` 专门为解决这个问题提供了 `?` 操作符。

- 在 `write!` 上使用 `?` 会像是这样：
```rust
// 对 `write!` 进行尝试（try），观察是否出错。若发生错误，返回相应的错误。
// 否则（没有出错）继续执行后面的语句。
write!(f, "{}", value)?;
```
- 有了 `?`，对一个 `Vec` 实现 `fmt::Display` 就很简单了：
```rust
use std::fmt; // 导入 `fmt` 模块。

// 定义一个包含单个 `Vec` 的结构体 `List`。
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 使用元组的下标获取值，并创建一个 `vec` 的引用。
        let vec = &self.0;

        write!(f, "[")?;

        // 使用 `v` 对 `vec` 进行迭代，并用 `count` 记录迭代次数。
        for (count, v) in vec.iter().enumerate() {
            // 对每个元素（第一个元素除外）加上逗号。
            // 使用 `?` 或 `try!` 来返回错误。
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }

        // 加上配对中括号，并返回一个 fmt::Result 值。
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
```