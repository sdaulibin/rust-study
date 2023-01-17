use List::*;
use num::complex::Complex;
enum List {
    // Cons：元组结构体，包含链表的一个元素和一个指向下一节点的指针
    Cons(u32,Box<List>),
    Nil,
}

impl List {
    // 创建一个空的 List 实例
    fn new() -> List{
        // `Nil` 为 `List` 类型（译注：因 `Nil` 的完整名称是 `List::Nil`）
        Nil
    }
    // 处理一个 List，在其头部插入新元素，并返回该 List
    fn prepend(self,element: u32) -> List{
        Cons(element,Box::new(self))
    }
    fn len(&self) -> u32{
        match *self {
            Cons(_,ref tail) => 1+tail.len(),
            Nil => 0
        }
    }

    fn stringfy(&self) -> String {
        match &self {
            Cons(head, ref tail) => {
                format!("{}, {}",head,tail.stringfy())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}
fn main() {
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringfy());

    let penguin_data = "\
   common name,length (cm)
   Little penguin,33
   Yellow-eyed penguin,65
   Fiordland penguin,60
   Invalid,data
   ";
    let records = penguin_data.lines();
   
    for (i,record) in records.enumerate() {
        if i==0 || record.trim().len()==0 {
            continue;
        }
        let fields: Vec<_> = record.split(",")
            .map(|field| field.trim())
            .collect();
        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields)
        }
        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            // 输出到标准输出
            println!("{}, {}cm", name, length);
        }
   }

   let a = Complex{re: 2.1 , im: -1.2};
   let b = Complex::new(11.1, 22.2);
   let result = a + b;
   println!("{},{}i",result.re,result.im);
}
