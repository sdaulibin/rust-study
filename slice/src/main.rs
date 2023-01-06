fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[..];
    println!("the slice word is : {},{}",hello,world);

    let word = first_word(&s);
    //cannot borrow `s` as mutable because it is also borrowed as immutable
    //s.clear();
    println!("the first word is : {}",word);
}
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate()  {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
