use std::ops::{Add, Mul};

use num::traits::AsPrimitive;

/// Intended to be used with: .iter().fold(num, fold_decimal)
pub fn fold_decimal<T>(acc: T, chr: &u8) -> T
where
    T: Copy + 'static,
    T: Add<Output = T>,
    T: Mul<Output = T>,
    u8: num::traits::AsPrimitive<T>,
{
    acc * 10.as_() + (chr - b'0').as_()
}

/// Intended to be used with: .iter().fold(num, fold_decimal)
pub fn fold_decimal_from<T>(number: &[u8]) -> T
where
    T: Copy + 'static,
    T: Add<Output = T>,
    T: Mul<Output = T>,
    u8: num::traits::AsPrimitive<T>,
{
    number.iter().fold(0.as_(), fold_decimal)
}

/// Function to convert a byte array into a integer of type T using a base of 2 through 36.
/// RADIX has to be between 2 and 36, or it will runtime panic.
/// function ignores junk.
pub fn atoi<T, const RADIX: u8>(number: &[u8]) -> T
where
    T: Copy + 'static,
    T: Add<Output = T>,
    T: Mul<Output = T>,
    u8: num::traits::AsPrimitive<T>,
{
    assert!(RADIX > 1 && RADIX < 37);
    number.iter().fold(0.as_(), |acc, &chr| {
        let val = (match chr {
            b'0'..=b'9' => chr,
            b'A'..=b'Z' => chr - 7,  // b'9' to b'A' has 7 other chars between
            b'a'..=b'z' => chr - 39, // b'9' to b'a' has 39 other chars between it.
            _ => b'0',               // junk.
        } - b'0');
        // e.g. if radix is 2, set to 0 if the value b'3' is found.
        let val = if RADIX <= val { 0 } else { val };
        acc * RADIX.as_() + val.as_()
    })
}

/// Function to convert a byte array into a integer of type T using a base of 2 through 36.
/// RADIX has to be between 2 and 36, or it will runtime panic.
/// function returns None on failure.
pub fn try_atoi<T, const RADIX: u8>(number: &[u8]) -> Option<T>
where
    T: Copy + 'static,
    T: Add<Output = T>,
    T: Mul<Output = T>,
    u8: num::traits::AsPrimitive<T>,
{
    assert!(RADIX > 1 && RADIX < 37);
    number.iter().try_fold(0.as_(), |acc, &chr| {
        let val = (match chr {
            b'0'..=b'9' => chr,
            b'A'..=b'Z' => chr - 7,  // b'9' to b'A' has 7 other chars between
            b'a'..=b'z' => chr - 39, // b'9' to b'a' has 39 other chars between it.
            _ => 255,                // junk.
        } - b'0');
        // e.g. if radix is 2, set to 0 if the value b'3' is found.
        if RADIX <= val {
            return None;
        }
        Some(acc * RADIX.as_() + val.as_())
    })
}
