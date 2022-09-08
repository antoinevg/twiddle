#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
#[macro_use]
extern crate std;

pub use core::ops::Range;

/// Return the bit width of the implementing type
pub trait BitWidth {
    const WIDTH: u8;
    fn width() -> u8;
}

impl BitWidth for u32 {
    const WIDTH: u8 = u32::max_value().leading_ones() as u8;
    fn width() -> u8 {
        Self::WIDTH
    }
}

/// Get the value of the bit at `index`
#[inline(always)]
pub fn bit(input: u32, index: u8) -> bool {
    assert!(index < u32::WIDTH);
    input & (1 << index) != 0
}

/// Set the value of the bit at `index` to `value`
#[inline(always)]
pub fn set(input: u32, index: u8, value: bool) -> u32 {
    assert!(index < u32::WIDTH);
    let mask = u32::MIN.wrapping_sub(value as u32) ^ input;
    input ^ mask & (1 << index)
}

/// Return value of the bits in the given range
#[inline(always)]
pub fn range(input: u32, r: Range<u8>) -> u32 {
    assert!(r.start < r.end);
    assert!(r.end < u32::WIDTH);
    let shift_left = u32::WIDTH - (r.end + 1);
    let shift_right = r.start + shift_left;
    (input << shift_left) >> shift_right
}

/// Set the value of the bits in the given range
#[inline(always)]
pub fn set_range(input: u32, r: Range<u8>, value: u32) -> u32 {
    assert!(r.start < r.end);
    assert!(r.end < u32::WIDTH);
    let mask = !(range(u32::MAX, r.start..r.end) << r.start);
    input & mask | value << r.start
}

/*
// - operate directly on register -----------------------------------------

/// Set the register value of the bit at `index` to `value`
///
/// # Safety
///
/// Is this the pointer you were looking for?
#[inline(always)]
pub unsafe fn reg_set(p: *mut u32, index: usize, bit: bool) {
    let mask = 1 << index;
    if bit {
        *p |= mask;
    } else {
        *p &= !mask;
    }
}

/// Get the register value of the bit at `index`
///
/// # Safety
///
/// Is this the pointer you were looking for?
#[inline(always)]
pub unsafe fn reg_toggle(p: *mut u32, index: usize) {
    let mask = 1 << index;
    *p ^= mask;
}

/// Check if the bit in the register at `index` is set
///
/// # Safety
///
/// Is this the pointer you were looking for?
#[inline(always)]
pub unsafe fn reg_is_set(r: *const u32, index: usize) -> bool {
    (*r & 1 << index) != 0
}

/// Check if the bit in the register at `index` is clear
///
/// # Safety
///
/// Is this the pointer you were looking for?
#[inline(always)]
pub unsafe fn reg_is_clear(r: *const u32, index: usize) -> bool {
    (*r & 1 << index) == 0
}
*/

#[cfg(test)]
mod tests {
    use crate as twiddle;

    const TEST_PATTERN: u32 = 0b1000_0110_0101_0100_0011_0010_0001_0000;

    #[test]
    fn test_bit() {
        let bits = TEST_PATTERN;
        assert_eq!(twiddle::bit(bits, 00), false);
        assert_eq!(twiddle::bit(bits, 01), false);
        assert_eq!(twiddle::bit(bits, 02), false);
        assert_eq!(twiddle::bit(bits, 03), false);
        assert_eq!(twiddle::bit(bits, 04), true);
        assert_eq!(twiddle::bit(bits, 05), false);
        assert_eq!(twiddle::bit(bits, 06), false);
        assert_eq!(twiddle::bit(bits, 07), false);
        assert_eq!(twiddle::bit(bits, 08), false);
        assert_eq!(twiddle::bit(bits, 09), true);
        assert_eq!(twiddle::bit(bits, 10), false);
        assert_eq!(twiddle::bit(bits, 11), false);
        assert_eq!(twiddle::bit(bits, 12), true);
        assert_eq!(twiddle::bit(bits, 13), true);
        assert_eq!(twiddle::bit(bits, 14), false);
        assert_eq!(twiddle::bit(bits, 15), false);
        assert_eq!(twiddle::bit(bits, 16), false);
        assert_eq!(twiddle::bit(bits, 17), false);
        assert_eq!(twiddle::bit(bits, 18), true);
        assert_eq!(twiddle::bit(bits, 19), false);
        assert_eq!(twiddle::bit(bits, 20), true);
        assert_eq!(twiddle::bit(bits, 21), false);
        assert_eq!(twiddle::bit(bits, 22), true);
        assert_eq!(twiddle::bit(bits, 23), false);
        assert_eq!(twiddle::bit(bits, 24), false);
        assert_eq!(twiddle::bit(bits, 25), true);
        assert_eq!(twiddle::bit(bits, 26), true);
        assert_eq!(twiddle::bit(bits, 27), false);
        assert_eq!(twiddle::bit(bits, 28), false);
        assert_eq!(twiddle::bit(bits, 29), false);
        assert_eq!(twiddle::bit(bits, 30), false);
        assert_eq!(twiddle::bit(bits, 31), true);
    }

    #[test]
    fn test_set() {
        let bits = TEST_PATTERN;

        let bits = twiddle::set(bits, 04, false);
        assert_eq!(twiddle::range(bits, 0..11), 0b0010_0000_0000);
        let bits = twiddle::set(bits, 09, false);
        let bits = twiddle::set(bits, 12, false);
        let bits = twiddle::set(bits, 13, false);
        let bits = twiddle::set(bits, 18, false);
        let bits = twiddle::set(bits, 20, false);
        let bits = twiddle::set(bits, 22, false);
        assert_eq!(twiddle::range(bits, 20..31), 0b1000_0110_0000);
        let bits = twiddle::set(bits, 25, false);
        let bits = twiddle::set(bits, 26, false);
        let bits = twiddle::set(bits, 31, false);
        assert_eq!(bits, 0);

        let bits = twiddle::set(bits, 04, true);
        assert_eq!(twiddle::range(bits, 0..11), 0b0000_0001_0000);
        let bits = twiddle::set(bits, 09, true);
        let bits = twiddle::set(bits, 12, true);
        let bits = twiddle::set(bits, 13, true);
        let bits = twiddle::set(bits, 18, true);
        let bits = twiddle::set(bits, 20, true);
        let bits = twiddle::set(bits, 22, true);
        assert_eq!(twiddle::range(bits, 20..31), 0b0000_0000_0101);
        let bits = twiddle::set(bits, 25, true);
        let bits = twiddle::set(bits, 26, true);
        let bits = twiddle::set(bits, 31, true);
        assert_eq!(bits, TEST_PATTERN);
    }

    #[test]
    fn test_range() {
        let bits = TEST_PATTERN;

        assert_eq!(twiddle::range(bits, 00..03), 0b0000);
        assert_eq!(twiddle::range(bits, 04..07), 0b0001);
        assert_eq!(twiddle::range(bits, 08..11), 0b0010);
        assert_eq!(twiddle::range(bits, 12..15), 0b0011);
        assert_eq!(twiddle::range(bits, 16..19), 0b0100);
        assert_eq!(twiddle::range(bits, 20..23), 0b0101);
        assert_eq!(twiddle::range(bits, 24..27), 0b0110);
        assert_eq!(twiddle::range(bits, 28..31), 0b1000);

        assert_eq!(twiddle::range(bits, 20..22), 0b101);
        assert_eq!(twiddle::range(bits, 21..23), 0b010);
    }

    #[test]
    fn test_set_range() {
        let bits = 0;

        let bits = twiddle::set_range(bits, 00..03, 0b0000);
        let bits = twiddle::set_range(bits, 04..07, 0b0001);
        let bits = twiddle::set_range(bits, 08..11, 0b0010);
        let bits = twiddle::set_range(bits, 12..15, 0b0011);
        let bits = twiddle::set_range(bits, 16..19, 0b0100);
        let bits = twiddle::set_range(bits, 20..23, 0b0101);
        let bits = twiddle::set_range(bits, 24..27, 0b0110);
        let bits = twiddle::set_range(bits, 28..31, 0b1000);
        assert_eq!(bits, TEST_PATTERN);

        let bits = !bits;

        let bits = twiddle::set_range(bits, 00..03, 0b0000);
        let bits = twiddle::set_range(bits, 04..07, 0b0001);
        let bits = twiddle::set_range(bits, 08..11, 0b0010);
        let bits = twiddle::set_range(bits, 12..15, 0b0011);
        let bits = twiddle::set_range(bits, 16..19, 0b0100);
        let bits = twiddle::set_range(bits, 20..23, 0b0101);
        let bits = twiddle::set_range(bits, 24..27, 0b0110);
        let bits = twiddle::set_range(bits, 28..31, 0b1000);
        assert_eq!(bits, TEST_PATTERN);
    }
}
