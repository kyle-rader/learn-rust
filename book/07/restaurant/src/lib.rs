mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() {}
        pub fn order_up() {}
        pub fn serve_order() {}
    }
}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
        Fries,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn cook_order() {}

    fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::serving::serve_order();
    }
}

pub fn eat() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast
    let mut meal = back_of_house::Breakfast::summer("Wheat");
    meal.toast = String::from("Sour Dough");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Salad;
}