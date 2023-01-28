struct Sheep {
    naked: bool,
    name: String
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }
    fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is already naked ...",self.name);
        } else {
            println!("{} gets a haircut!",self.name);
            self.naked = true;
        }
    }
}

trait Animal {
    fn new(name: String) -> Self;
    fn name(&self) -> String;
    fn noise(&self) -> String;
    fn talk(&self) {
        println!("{} says {} ...",self.name(),self.noise());
    }
}

impl Animal for Sheep {
    fn new(name: String) -> Sheep {
        Sheep { naked: false, name: name }
    }
    fn name(&self) -> String {
        self.name.clone()
    }
    fn noise(&self) -> String {
        if self.is_naked() {
            "baaaaah?".to_string()
        } else {
            "baaaaah!".to_string()
        }
    }
    fn talk(&self) {
        println!("{} pauses briefly... {} ...",self.name(),self.noise());
    }
}

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x:f64,y :f64) ->Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1 : Point,
    p2 : Point,
}

impl Rectangle {
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1} = self.p1;
        let Point { x: x2, y: y2} = self.p2;
        ((x1 - x2) * (y1 - y2)).abs()
    }
    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }
    fn translate(&mut self,x:f64,y:f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

struct Pairs (Box<i32>,Box<i32>);

impl Pairs {
    fn destroy(self) {
        let Pairs(first,second) = self;
        println!("Destroying Pair({}, {})", first, second);
    }
}

fn main() {
    println!("Hello, world!");
    let mut dolly: Sheep = Animal::new("Dolly".to_string());
    dolly.talk();
    dolly.shear();
    dolly.talk();

    let rectangle = Rectangle {
        // 静态方法使用双冒号调用
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // 实例方法通过点运算符来调用
    // 注意第一个参数 `&self` 是隐式传递的，亦即：
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // 报错！ `rectangle` 是不可变的，但这方法需要一个可变对象
    //rectangle.translate(1.0, 0.0);
    // 试一试 ^ 去掉此行的注释

    // 正常运行！可变对象可以调用可变方法
    square.translate(1.0, 1.0);

    //let pair = Pair(Box::new(1), Box::new(2));

    //pair.destroy();
}
