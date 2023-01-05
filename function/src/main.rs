fn main() {
    anther_fn(143,233);

    let y = {
        let x = 3;
        x+2
    };
    println!("thse value of y is: {}",y);

    let sum = plus_two(99, 98);
    println!("the value of sum is : {}",sum);

    control();

    fn_while();

    fun_for();
}

fn anther_fn(x: u32,y: i32) {
    println!("the value of x,y is : {},{}",x,y);
}

fn plus_two(x:i32,y: i32) -> i32 {
    x+y
}

fn control() {
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 2,3,4");
    }
}

fn fn_while() {
    let a = [1,2,3,4,5,6,7,8,9,10];
    let mut index = 0;
    while index < 8 {
        println!("the values is : {}",a[index]);
        index += 1;
    }
}

fn fun_for() {
    let a = [1,2,3,4,5,6,7,8,9,10];
    for ele in a {
        println!("ths value is : {}",ele);
    }
}