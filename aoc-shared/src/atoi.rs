use std::ops::{Add, Mul};

use num::{traits::AsPrimitive, CheckedAdd, CheckedMul};

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
            _ => return acc,         // junk.
        } - b'0');
        // e.g. if radix is 2, b'3' would cause an early return so to ignore the value.
        let val = if RADIX <= val { return acc } else { val };
        acc * RADIX.as_() + val.as_()
    })
}

/// Function to convert a byte array into a integer of type T using a base of 2 through 36.
/// RADIX has to be between 2 and 36, or it will runtime panic.
/// function returns None on failure.
pub fn try_atoi<T, const RADIX: u8>(number: &[u8]) -> Option<T>
where
    T: Copy + 'static,
    T: CheckedAdd<Output = T>,
    T: CheckedMul<Output = T>,
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
        Some(RADIX.as_().checked_mul(&acc)?.checked_add(&val.as_())?)
    })
}

/// twenty six (a-z or A-Z) radix number. no 0-9 digits, thus not for atoi.
/// made const since this is a very special number format for weird number problems.
pub const fn ts_to_u64(number: &[u8]) -> u64 {
    let mut acc = 0u64;
    let mut i = 0usize;
    while i < number.len() {
        let digit = number[i];
        let digit = match digit {
            b'A'..=b'Z' => digit - b'A',
            b'a'..=b'z' => digit - b'a',
            _ => {
                i += 1; // skip junk
                continue;
            }
        } as u64;
        acc = acc * 26 + digit;
        i += 1;
    }
    acc
}

const TS_DIGITS: &[u8; 26] = b"abcdefghijklmnopqrstuvwxyz";

/// twenty six (a-z or A-Z) radix number. no 0-9 digits, thus not for atoi.
/// made const since this is a very special number format for weird number problems.
pub const fn u64_to_ts<const N: usize>(number: u64) -> [u8; N] {
    let mut acc = [0u8; N];
    let mut i = 0usize;
    while i < N {
        let digit = number / 26u64.pow(i as u32) % 26;
        acc[(N - 1) - i] = TS_DIGITS[digit as usize];
        i += 1;
    }
    acc
}

#[cfg(test)]
mod test {
    use crate::{ts_to_u64, u64_to_ts};

    #[test]
    fn ts_to_u64_is_u64_to_ts() {
        let n = ts_to_u64(b"tsrcxpeh");
        let nprime = b"tsrcxpeh";
        assert_eq!(&u64_to_ts(n), nprime);
    }
}
