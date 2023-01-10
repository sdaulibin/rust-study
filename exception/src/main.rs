use std::fs::File;
use std::io;
use std::io::Read;
use std::io::ErrorKind;

fn main() {
    println!("Hello Word");
    // panic!("crash & burn");
    // first();
    /*let file = File::open("hello.md");
    let file = File::open("hello.md").expect("无法打开文件");*/
    /* match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(error) => panic!("无法创建文件:{:?}",error),
            },
            other_error => {
                panic!("无法打开文件: {:?}",error)
            }
        },
    };*/
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

//测试RUST_BACKTRACE=1 cargo run
/* fn first() {
    second();
}

fn second() {
    third(33);
}
 
fn third(num: i32) {
    if num == 33 {
        panic!("请不要输入33");
    }
}*/

fn read_username_from_file() -> Result<String, io::Error>{
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match  f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}