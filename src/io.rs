use::bitpack::*;
use std::io::{stdin, stdout};

// remember to flush!

/*
    Out - output the value in register C
    Arguments: reference to registers array, current instruction word
    Returns: 0 on success, 101-109 on failure
*/
#[inline]
pub fn out(r: &mut [u32; 8], iw: u32) -> u32 {

    return 0;
}

/* 
    Inp - read a value from the user and store it in register C
    Arguments: reference to registers array, current instruction word
    Returns: 0 on success, 101-109 on failure
*/
#[inline]
pub fn inp(r: &mut [u32; 8], iw: u32) -> u32 {

    return 0;
}