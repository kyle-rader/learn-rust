//! # kinds
//!
//! Enums for representing colors!

/// The primary colors by the RYB color model.
#[derive(Debug)]
pub enum PrimaryColor {
    Red,
    Yellow,
    Blue,
}

/// The secondary colors by the RYB color model.
#[derive(Debug)]
pub enum SecondaryColor {
    Orange,
    Green,
    Purple,
}
