struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn y(&self) -> f64 {
        self.y
    }
}

fn main() {
    let number_list = vec![34,50,65,12,123,465,78];
    let largest = get_largest_int(number_list);
    println!("the largest number is : {}",largest);
    let number_list = vec![34,50,999,345,123,465,78];
    let largest = get_largest(number_list);
    println!("the largest number is : {}",largest);

    let char_list = vec!['a','q','w','e','r'];
    let largest = get_largest_char(char_list);
    println!("the largest char is : {}",largest);
    let char_list = vec!['a','q','w','e','r','x','z'];
    let largest = get_largest(char_list);
    println!("the largest char is : {}",largest);

    let p1 = Point{x:5,y:10};
    println!("p.x = {}",p1.x());
    let p2 =  Point{x:5.0,y:10.02};
    println!("p.y = {}",p2.y());
}

fn get_largest_int(number_list: Vec<i32>) -> i32 {
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn get_largest_char(char_list: Vec<char>) -> char {
    let mut largest = char_list[0];
    for number in char_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T{
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}