fn main() {
    let x = 5;
    println!("the value of x is: {}",x);
    let x = 6; //将x变量隐藏
    println!("the value of x is: {}",x);
    let x = x + 1;
    println!("the value of x is: {}",x);
    let x =  x * 3;
    println!("the value of x is: {}",x);

    let y = 2.0;
    let z:f32 = 3.0;

    let sum = y+z;
    let product = y/z;

    println!("result:{},{}",sum,product);

    let tup:(i32,f64,u8) = (500,6.4,2);
    println!("this value is:{},{},{}",tup.0,tup.1,tup.2);

    let (m,n,h) = tup;

    println!("the value of m,n,h is: {},{},{}",m,n,h);

    let a = [1,2,3,4,5];

    let b = [4;5];

    let months = ["January","February","March","April"];

    println!("the value is : {},{}",b[1],a[4]);
    println!("the value is : {}",b[4]);
    println!("the value is : {}",months[3]);
}
