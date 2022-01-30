pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for comp in self.components.iter() {
            comp.draw();
        }
    }
}

#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Button {
    pub fn new(label: String) -> Button {
        Button {
            width: 8,
            height: 4,
            label,
        }
    }
}

impl Draw for Button {
    fn draw(&self) {
        // pretend to draw
        println!("A {:?} is drawn to the screen!", self);
    }
}

