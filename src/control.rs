use::bitpack::*;

/*
    Cmov - conditional move
    Arguments: reference to registers array, current instruction word
    Returns: 0 on success, 1-9 on failure
*/
#[inline]
pub fn cmov(r: &mut [u32; 8], iw: u32) -> u32 {

    return 0;
}

/*
    Halt - halt the program
    Arguments: none
    Returns: 70 to signal program halt, 71-79 on failure (which should be impossible)
*/
#[inline]
pub fn halt() -> u32 {

    return 70;
}