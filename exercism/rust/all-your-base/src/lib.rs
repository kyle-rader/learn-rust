#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
    Overflow(String),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
/// What is a number?
///
/// 1  0  1 - base 2 (5)
/// |  |  |
/// |  |  1 * 2^0 (1) = 1
/// |  | 0 * 2^1 (2) = 0
/// | 1 * 2^2 (4) = 4
/// == 5
///
/// Given 23 (b10) go to b2.
/// n = 23.
/// 
///             1
/// __ __ __ __ __
/// 1  2  4  8  16
/// 
/// (23 / 2) -> 11  (11 / 2) -> 5 -> (5 / 2) -> 2 (2/2) -> 0
/// exp = 4
/// 23 % 2^4 = 7
/// 7 % 2^3 = 7
/// 23 - (2^ 4) = 7
/// 
/// 
/// 

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    let mut n = 0;
    for (i, val) in number.iter().rev().enumerate() {
        if let Some(exp) = from_base.checked_pow(i as u32) {
            n += val * exp
        } else {
            return Err(Error::Overflow(format!(
                "failed to raise {from_base} to the power of {i}."
            )));
        }
    }

    let mut result: Vec<u32> = Vec::new();
    let mut i = 0;

    eprintln!("Starting convert {n} (base 10) into base {to_base}");
    while n > 0 {
        if let Some(exp) = to_base.checked_pow(i) {
            eprintln!("n is {n}");
            eprintln!("i: {i}, exp: {exp}");

            let digit = ;
            eprintln!("rem of {n} % {exp} is {digit}");

            result.insert(0, digit);

            n -= digit * exp;
        } else {
            return Err(Error::Overflow(format!(
                "failed to raise {to_base} to the power of {i}."
            )));
        }
        i += 1;
    }
    Ok(result)
}
