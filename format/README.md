### 打印操作由 `std::fmt` 里面所定义的一系列宏来处理，包括：
- `format!`：将格式化文本写到字符串。
- `print!`：与 `format!` 类似，但将文本输出到控制台（`io::stdout`）。
- `println!`: 与 `print!` 类似，但输出结果追加一个换行符。
- `eprint!`：与 `print!` 类似，但将文本输出到标准错误（`io::stderr`）。
- `eprintln!`：与 `eprint!` 类似，但输出结果追加一个换行符。

### `std::fmt` 包含多种 trait（特质）来控制文字显示，其中重要的两种 trait 的基本形式如下：
- `fmt::Debug`：使用 `{:?}` 标记。格式化文本以供调试使用。
- `fmt::Display`：使用 `{}` 标记。以更优雅和友好的风格来格式化文本。
