mod front_of_house;
mod back_of_house;

pub use crate::front_of_house::hosting;


pub fn eat() {
    // absolute path
    // crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    hosting::add_to_waitlist();

    // Order a breakfast
    let mut meal = back_of_house::Breakfast::summer("Wheat");
    meal.toast = String::from("Sour Dough");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Salad;
}