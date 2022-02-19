use core::fmt;

struct KilometersNewType(f32);

impl fmt::Display for KilometersNewType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} km", self.0)
    }
}

type KilometersTypeAlias = f32;

fn main() {
    let marathon_distance = KilometersNewType(5.0);
    println!("Marathon distance: '{}'", marathon_distance);

    let ten_k_distance: KilometersTypeAlias = 10.0;
    println!("a 10K is: '{}' km", ten_k_distance + 0.02); // Can add f32 to KilometersTypeAlias
}
