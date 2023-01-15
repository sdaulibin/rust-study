use std::fmt; // 导入 `fmt`
fn main() {
    // 通常情况下，`{}` 会被任意变量内容所替换。
    // 变量内容会转化成字符串。
    println!("{} days", 31);
    // 不加后缀的话，31 就自动成为 i32 类型。
    // 你可以添加后缀来改变 31 的类型（例如使用 31i64 声明 31 为 i64 类型）。
    
    // 用变量替换字符串有多种写法。
    // 比如可以使用位置参数。
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    
    // 可以使用命名参数。
    println!("{subject} {verb} {object}",object="the lazy dog",subject="the quick brown fox",verb="jumps over");

    // 可以在 `:` 后面指定特殊的格式。
    println!("{} of {:b} people know binary, the other half don't", 1, 2);
    
    // 你可以按指定宽度来右对齐文本。
    // 下面语句输出 "     1"，5 个空格后面连着 1。
    println!("{number:>width$}", number=1, width=6);
    
    // 你可以在数字左边补 0。下面语句输出 "000001"。
    println!("{number:>0width$}", number=1, width=6);
    
    // println! 会检查使用到的参数数量是否正确。
    println!("My name is {0}, {1} {0}", "Bond" ,"James");
    // 改正 ^ 补上漏掉的参数："James"
    
    // 创建一个包含单个 `i32` 的结构体（structure）。命名为 `Structure`。
    //#[allow(dead_code)]
    //struct Structure(i32);
    // 但是像结构体这样的自定义类型需要更复杂的方式来处理。
    // 下面语句无法运行。
    //println!("This struct `{}` won't print...", Structure(3));
    // 改正 ^ 注释掉此行。

    #[derive(Debug)]
    struct Structure(i32);

    #[derive(Debug)]
    struct Deep(Structure);

    // 使用 `{:?}` 打印和使用 `{}` 类似。
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure` 也可以打印！
    println!("Now {:?} will print!", Structure(3));
    
    // 使用 `derive` 的一个问题是不能控制输出的形式。
    // 假如我只想展示一个 `7` 怎么办？
    println!("Now {:?} will print!", Deep(Structure(7)));

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8
    }
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    // 美化打印
    println!("{:#?},\n{1:?},{2:?}", peter,peter.name,peter.age);

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

    // 定义一个包含单个 `Vec` 的结构体 `List`。
    struct List(Vec<i32>);
    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // 使用元组的下标获取值，并创建一个 `vec` 的引用。
            let vec = &self.0;
            write!(f,"[")?;
            for (count,v) in vec.iter().enumerate() {
                // 对每个元素（第一个元素除外）加上逗号。
                // 使用 `?` 或 `try!` 来返回错误。
                if count !=0  {
                    write!(f,", ")?;
                }
                write!(f,"{0}:{1}",count,v)?;
            }
            // 加上配对中括号，并返回一个 fmt::Result 值。
            write!(f, "]")
        }
    }
    let v = List(vec![1,2,3,4,5,6,7,8,9,10]);
    println!("{}",v);
}
