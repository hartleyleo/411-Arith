use std::{convert::TryInto, arch::x86_64::_CMP_TRUE_UQ};


/// Returns true iff the signed value `n` fits into `width` signed bits.
/// 
/// # Arguments:
/// * `n`: A signed integer value
/// * `width`: the width of a bit field
pub fn fitss(n: i64, width: u64) -> bool {
    
    // Check if the n value is within the upper and lower bounds
    // since it is a signed integer
    if n >= -((1 << width) / 2) && n < ((1 << width) / 2){
        true
    }
    else{
        false
    }
}

/// Returns true iff the unsigned value `n` fits into `width` unsigned bits.
/// 
/// # Arguments:
/// * `n`: An usigned integer value
/// * `width`: the width of a bit field
pub fn fitsu(n: u64, width: u64) -> bool {

    // Check if the n value is within 0 and the upper bound
    // since it is an unsigned integer
    if n < (1 << width){
        true
    }
    else{
        false
    }
}

/// Retrieve a signed value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn gets(word: u64, width: u64, lsb: u64) -> i64 {
    let bitshft = 64 - width;
    (word << (bitshft - lsb)) as i64 >> (bitshft)
}

/// Retrieve an unsigned value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn getu(word: u64, width: u64, lsb: u64) -> u64 {
    let bitshft = 64 - width;
    (word << (bitshft - lsb)) >> (bitshft)    
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the unsigned `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` unsigned bits.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the unsigned value to place into that bit field
pub fn newu(word: u64, width: u64, lsb: u64, value: u64) -> Option<u64> {
    if fitsu(value, width){
        Some(value << lsb | word)
    }
    else{
        None
    }
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the signed `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` signed bits.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the signed value to place into that bit field
pub fn news(word: u64, width: u64, lsb: u64, value: i64) -> Option<u64> {
    if fitss(value, width){
        Some(((((!(-1_i64 << width)) & value) as i64) as u64) << lsb | word)
    }
    else{
        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fitss_range_test() {
        assert_eq!(fitss(7, 5), true);
        assert_eq!(fitss(-4, 3), true);
        assert_eq!(fitss(9001, 5), false);
        assert_eq!(fitss(31, 5), false);
    }

    #[test]
    fn fitsu_range_test() {
        assert_eq!(fitsu(17, 5), true);
        assert_eq!(fitsu(7, 3), true);
        assert_eq!(fitsu(9001, 5), false);
        assert_eq!(fitsu(31, 5), true);    
    }

    #[test]
    fn gets_test() {
        let test_case: i64 = gets(!0_u64, 64, 0);
        assert_eq!(test_case, -1_i64);
        assert_eq!(gets(9998212, 24, 6), 156222);
    }

    #[test]
    fn getu_test() {
        let test_case: u64 = getu(!0_u64, 64, 0);
        assert_eq!(test_case, !0_u64);
        assert_eq!(getu(9998212, 24, 6), 156222);
    }

    #[test]
    fn newu_test() {
        assert_eq!(newu(255, 4, 1, 7 as u64).unwrap(), 255);
    }
    
    #[test]
    fn news_test() {
        assert_eq!(news(0_u64, 9, 23, -0.98 as i64).unwrap(), 0);
    }
}