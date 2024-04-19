use crate::word::*;

/// Conditionally move a value from one register to another
/// if the value in the third register is not zero.
/// 
/// # Arguments
/// 
/// * `r` - a mutable reference to the array of registers
/// * `iw` - the current instruction word
/// 
/// # Returns
/// 
/// 0 on success, 1-9 on failure
#[inline]
pub fn cmov(r: &mut [u32; 8], iw: u32) -> u32 {
    // if R[C] != 0, R[A] := R[B]
    let abc:[usize; 3] = regs_array(iw);
    if r[abc[2]] != 0 {
        r[abc[0]] = r[abc[1]];
    }
    return 0;
}

/// Halts the program 
/// 
/// # Arguments
/// 
/// * None
/// 
/// # Returns
/// 
/// 70 to signal program halt, 71-79 on failure (which should be impossible)
#[inline]
pub fn halt() -> u32 {
    return 70;
}