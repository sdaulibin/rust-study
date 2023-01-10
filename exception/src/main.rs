use std::{fs::File, io::ErrorKind};

fn main() {
    println!("Hello Word");
    // panic!("crash & burn");
    // first();
    //let file = File::open("hello.md");
    // match file {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(error) => panic!("无法创建文件:{:?}",error),
    //         },
    //         other_error => {
    //             panic!("无法打开文件: {:?}",error)
    //         }
    //     },
    // };
    //闭包
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("无法创建文件：{:?}",error)
            })
        } else {
            panic!("无法打开文件：{:?}",error)
        }
    });
}

fn first() {
    second();
}

fn second() {
    third(33);
}
 
fn third(num: i32) {
    if num == 33 {
        panic!("请不要输入33");
    }
}