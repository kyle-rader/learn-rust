use gui::{Button, Draw, Screen};

#[derive(Debug)]
struct SelectBox {
    w: u32,
    h: u32,
    options: Vec<String>,
}

impl SelectBox {
    pub fn new(options: Vec<String>) -> SelectBox {
        SelectBox {
            w: 12,
            h: 4,
            options,
        }
    }
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("A {:?} is drawn to the screen!", self);
    }
}

// The folling does not compile because
/*
impl Draw for String {
   | ^^^^^^^^^^^^^^------
   | |             |
   | |             `String` is not defined in the current crate
   | impl doesn't use only types from inside the current crate
*/
// impl Draw for String {
//     fn draw(&self) {
//         println!("A String {:?} is drawn to the screen", self);
//     }
// }

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button::new(String::from("push me"))),
            Box::new(SelectBox::new(vec![
                String::from("Yes"),
                String::from("Maybe"),
                String::from("No"),
            ])),
            // [Cannot do this] Box::new(String::from("plain text component!")),
        ],
    };

    screen.run();
}
