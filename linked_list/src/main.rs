use List::*;
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
}
