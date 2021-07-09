mod art;

fn main() {
    println!("Hello, world!");

    let c1 = art::PrimaryColor::Blue;
    let c2 = art::PrimaryColor::Red;
    let c3 = art::mix(c1, c2);
    println!("We made {:?}!", c3);
}
