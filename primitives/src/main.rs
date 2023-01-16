use std::{fmt, mem};
//元组可以充当函数的参数和返回值
fn reverse(pair: (i32,bool)) -> (bool,i32) {
    //可以使用`let`把元组的成员绑定到一些变量
    let (integer,boolean) = pair;
    (boolean,integer)
}
#[derive(Debug)]
struct Matrix(f32,f32,f32,f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"( {},{} )\n( {},{} )",self.0,self.1,self.2,self.3)
    }
}

fn transpose(matrix: Matrix) -> Matrix{
    let result = Matrix(matrix.0,matrix.3,matrix.2,matrix.1);
    result
}

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}",slice[0]);
    println!("ths size has {} element",slice.len());
}
fn main() {
    // 整数相加
    println!("1 + 2 = {}", 1u32 + 2);

    // 整数相减
    println!("1 - 2 = {}", 1i32 - 2);
    // 试一试 ^ 尝试将 `1i32` 改为 `1u32`，体会为什么类型声明这么重要

    // 短路求值的布尔逻辑
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // 位运算
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // 使用下划线改善数字的可读性！
    println!("One million is written as {}", 1_000_000u32);

    let long_tuple = (1u8, 2u16, 3u32, 4u64,-1i8, -2i16, -3i32, -4i64,0.1f32, 0.2f64,'a', true);

    // 通过元组的下标来访问具体的值
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // 元组也可以充当元组的元素
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // 元组可以打印
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // 但很长的元组无法打印
    /* let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    println!("too long tuple: {:?}", too_long_tuple);*/

    let pair = (1,true);
    println!("the revesed pair is {:?}",reverse(pair));

    // 创建单元素元组需要一个额外的逗号，这是为了和被括号包含的字面量作区分。
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));
 
    // 元组可以被解构（deconstruct），从而将值绑定给变量
    let tuple = (1, "hello", 4.5, true);
 
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
 
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("{}",matrix);
     
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));

    // 定长数组（类型标记是多余的）
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // 所有元素可以初始化成相同的值
    let ys: [i32; 500] = [0; 500];

    // 下标从 0 开始
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` 返回数组的大小
    println!("array size: {}", xs.len());

    // 数组是在栈中分配的
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // 数组可以自动被借用成为 slice
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // slice 可以指向数组的一部分
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    // 越界的下标会引发致命错误（panic）
    //println!("{}", xs[5]);
}
