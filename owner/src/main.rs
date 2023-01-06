fn main() {
    let x = 32;
    let y = x; //拷贝 integer/boolean/charactor实现copy trait
    println!("the value of x,y:{},{}",x,y);

    let s1 = String::from("Hello");
    //let s2 = s1;
    let s2 = s1.clone();
    println!("the value of s1,s2:{},{}",s1,s2);

    let s = String::from("World");
    takes_ownership(s);
    // println!("ths value of s is : {}",s);

    let i = 5;
    makes_copy(i);
    println!("the value of i is : {}",i);

    let s = gives_ownship();
    println!("the value of s is {}",s);

    let s3 = String::from("Rust");
    let s4 = takes_gives_back(s3);
    println!("ths value of s4 : {}",s4);

    let s5 = String::from("Hello Rust");
    let (s6,len) = calculate_length(s5);
    println!("the length of {} is {}",s6,len);

    let s7 = String::from("Hello Go");
    let len2 = calculate_length_ref(&s7);
    println!("the length of {} is {}",s7,len2);

    let mut s8 = String::from("Hello");
    change_ref(&mut s8);
    println!("the value of s8 is {}",s8);

    // **限制1:在一个特定的作用域中，对某个特定的数据，只能有一个可变引用**
    let mut s9 = String::from("Hello!!");
    let r1: &String = &mut s9;
    //A mutable variable, reference, or pointer.
    //mut can be used in several situations. The first is mutable variables, which can be used anywhere you can bind a value to a variable name. Some examples:
    //let r2: &String = &mut s9;
    println!("ths value of r1 is : {}",r1);

    // **限制2:在一个特定作用域中，对于某个特定的不可变引用，不能再添加可变引用**
    let mut s10 = String::from("Hi i am rust");
    let r3: &String = &s10;
    let r4: &String = &s10;
    // cannot borrow `s10` as mutable because it is also borrowed as immutable
    //let r5: &mut String = &mut s10;
    println!("ths value of r3,r4 is : {},{}",r3,r4);
    let r5: &mut String = &mut s10;
    println!("ths value of r5 is : {}",r5);

    //在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
    //引用必须总是有效的。
}

fn takes_ownership(some_sting: String) {
    println!("the value of string is : {}",some_sting);
}

fn makes_copy(some_integer: i32) {
    println!("ths value of integer is {}",some_integer);
}

fn gives_ownship() ->String {
    let some_string = String::from("Hello World");
    some_string
}

fn takes_gives_back(a_string: String) -> String{
    a_string
}

fn calculate_length(s: String)-> (String,usize) {
    let length = s.len();
    (s,length)
}

fn calculate_length_ref(s: &String)-> usize {
    let length = s.len();
    length
}

fn change_ref(some_string: &mut String) {
    some_string.push_str(" World !!!");
}