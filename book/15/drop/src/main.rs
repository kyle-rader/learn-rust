struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    println!("Hello, drop!");
    let a = CustomSmartPointer {
        data: String::from("some stuff in a")
    };
    {
        let _b = CustomSmartPointer {
            data: String::from("some other stuff in b")
        };
    }

    drop(a);

    println!("Smart Pointers have been created, and are already dropped!");
}
