use rand::Rng;

mod front_of_house;
//{
    //简化版
    // pub mod hosting {
    //     pub fn add_to_waitlist() {
    //     }
    // }
//}

// pub fn eat_at_restaurant() {
//     crate::front_of_house::hosting::add_to_waitlist();
//     front_of_house::hosting::add_to_waitlist();
// }

// fn super_order() {
// }

mod back_of_house {
    // fn fix_incorret_order() {
    //     cook_order();
    //     super::super_order();
    // }
    // fn cook_order() {    
    // }
    pub enum Appetizer {
        Soup,
        Salad,
    }
    pub struct Breakfast {
        pub toast: String,
        pub seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast { 
                toast: String::from(toast),
                seasonal_fruit: String::from("梨"), 
            }
        }
    }
}

// use crate::front_of_house::hosting;
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("黑麦");
    meal.toast = String::from("小麦");

    let meal2 = back_of_house::Breakfast {
        toast: String::from("小麦"),
        seasonal_fruit: String::from("梨"),
    };

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    let secret_number = rand::thread_rng().gen_range(1..101);
}
