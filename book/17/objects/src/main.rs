pub struct Averager {
    count: i32,
    sum: i32,
    average: i32,
}

impl Averager {
    pub fn new() -> Averager {
        Averager {
            count: 0,
            sum: 0,
            average: 0,
        }
    }

    pub fn add(&mut self, x: i32) {
        self.count += 1;
        self.sum += x;
        self.average = self.sum / self.count;
    }

    pub fn average(& self) -> i32 {
        self.average
    }
}

fn main() {
    let mut avg = Averager::new();

    for i in 0..100 {
        avg.add(i);
    }
    println!("The average of numbers 0-99 is {}", avg.average());
}
