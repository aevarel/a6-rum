use std::io::{stdin, stdout, Read, Write};
use crate::word::*;

// remember to flush stdout after printing

/// Outputs the value held in register C to the console
/// 
/// # Arguments
/// 
/// * `r` - reference to the array of registers
/// * `iw` - the current instruction word
///
/// # Returns
/// 
/// 0 on success
#[inline]
pub fn out(r: &mut [u32; 8], iw: u32) -> u32 {

    // get the register
    let c = regs_array(iw)[2];

    // output the value in the register
    if r[c] <= 255 {
        print!("{}", r[c] as u8 as char);
        stdout().flush().unwrap();
    }

    return 0;
}

/// Reads a single character from the user and stores it in register C
/// 
/// # Arguments
/// 
/// * `r` - reference to the array of registers
/// * `iw` - the current instruction word
/// 
/// # Returns
/// 
/// 0 on success
#[inline]
pub fn inp(r: &mut [u32; 8], iw: u32) -> u32 {

    // get the register
    let c = regs_array(iw)[2];

    // read a single character from the user
    let mut input = [0u8; 1];
    stdin().read(&mut input).unwrap();
    r[c] = input[0] as u32;

    return 0;
}