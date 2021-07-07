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
