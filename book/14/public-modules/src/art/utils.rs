//! # utils
//! 
//! Helper methods for working with art concepts!

use super::kinds::*;

pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
    println!("Mixing {:?} and {:?}", c1, c2);
    SecondaryColor::Purple
}