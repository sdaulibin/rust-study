use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashMap;
fn main() {
    //vactor
    let a = [1,2,3,4,5,6,7,8,9,10];
    let mut v: Vec<i32> = Vec::new();
    v.push(11);
    v.push(12);
    v.push(13);

    let mut v2 = vec![14,15,16,17];

    let third = &v2[3];

    println!("the value of a is : {:?}",a);
    println!("the value of v is : {:?}",v);
    println!("the value of v2 is : {:?}",v2);
    println!("the value of third is : {}",third);

    match v.get(5) {
        Some(value) => println!("the value of element is {}",value),
        None => println!("there is no third element."),
    }

    for ele in &v2 {
        println!("the value of ele is : {}",ele)
    }

    for ele in &mut v2 {
        *ele += 100;
    }

    for ele in &v2 {
        println!("the value of ele is : {}",ele)
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    match &row[0] {
        SpreadsheetCell::Int(i) => println!("the value of i {}",i),
        _ => println!("not an integer"),
    }
    //String
    let mut s1 = String::new();
    let s2 = "initail contents";
    let s3 = s2.to_string();
    let s4= String::from("hello world");
    let s6 = format!("{} {}",s3,s4);
    let s5 = s3 + &s4;


    // let hello = String::from("السلام عليكم");
    // let hello = String::from("Dobrý den");
    //let hello = String::from("Hello");
    // let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते"); //南伍斯特
    // let hello = String::from("こんにちは");
    // let hello = String::from("안녕하세요");
    // let hello = String::from("你好");
    // let hello = String::from("Olá");
    // let hello = String::from("Здравствуйте");
    // let hello = String::from("Hola");

    s1.push_str("hello");
    s1.push_str(" world");
    s1.push('!');
    println!("{}",s1);
    println!("{}",s5);
    println!("{}",s6);

    //字节
    for b in hello.bytes() {
        println!("{}",b)
    }
    //scalar数量值（char）
    for c in hello.chars() {
        println!("{}",c)
    }
    //字形族
    for g in hello.graphemes(true) {
        println!("{}",g)
    }

    let blue = String::from("Blue");
    let yellow = String::from("Yellow");
    let mut scores = HashMap::new();

    scores.insert(blue, 10); //传入所有权
    scores.insert(yellow, 20);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    match score {
      Some(value) => println!("the value of score is {}",value),
      _ => println!("no value"),  
    };

    for (key,value) in &scores {
        println!("key {}, value {}",key,value)
    }

    scores.insert(String::from("Blue"), 30);
    scores.entry(String::from("Yellow")).or_insert(40);
    println!("the value of scores is {:#?}",scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("the value of map is {:?}",map)
}