pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("Read more from {} ...",self.summarize_author()) //缺省实现
    }
}
pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    /* fn summarize(&self) -> String {
        format!("{}, by {} ({})",self.headline,self.author,self.content)
    }*/
    fn summarize_author(&self) -> String {
        format!("{}",self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet  {
    fn summarize_author(&self) -> String {
        format!("@{}",self.username)
    }

    fn summarize(&self) -> String {
        format!("{},{}",self.username,self.content)
    }
}

//trait 作为参数
/* pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}",item.summarize())
}*/

//Trait Bound 泛型 特征限制
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}",item.summarize())
}

//多个参数
pub fn muti_impl<T: Summary>(item1: &T,item2: &T) {
    println!("Breaking news! {}",item1.summarize());
    println!("Breaking news! {}",item2.summarize());
}

/*fn retuen_summarizable(switch: bool) -> impl Summary{
    if switch {
        NewsArticle {
            author: String::from("John Doe"),
            headline: String::from("The sky is Failling!"),
            content: String::from("The sky is not actually failing."),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know,people"),
            reply: false,
            retweet: false,
        }
    }
}*/

fn largest<T: PartialOrd + Copy>(list:&[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know,people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet : {}",tweet.summarize());

    let article = NewsArticle {
        author: String::from("John Doe"),
        headline: String::from("The sky is Failling!"),
        content: String::from("The sky is not actually failing."),
    };
    println!("2 new article : {}",article.summarize());

    //notify(&article);
    //notify(&tweet);

    notify(&article);
    notify(&tweet);

    muti_impl(&article, &article);

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
