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