fn main() {
    let x: i8 = 4;
    let y = Some(12);
    let sum = x + y.unwrap_or(4);
    println!("the value of sum is :{}",sum);

    value_in_cents(Coin::Quarter(UsState::Alaska));

    let m = Some(10);
    let n = plus_one(m);
    let o = plus_one(None);
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizons,
    Arkanas,
    California,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny =>{
            println!("Lucky Penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}",state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i+1),
        //None => None,
        _ => None,
    }
}