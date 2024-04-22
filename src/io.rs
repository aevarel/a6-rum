use std::io::{stdin, stdout, Read, Write};
use crate::word::*;

// remember to flush!

/*
    Out - output the value in register C
    Arguments: reference to registers array, current instruction word
    Returns: 0 on success, 101-109 on failure
*/
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

/* 
    Inp - read a single character from the user and store it in register C
    Arguments: reference to registers array, current instruction word
    Returns: 0 on success, 101-109 on failure
*/
#[inline]
pub fn inp(r: &mut [u32; 8], iw: u32) -> u32 {

    // get the register
    let c = regs_array(iw)[2];

    // read a single character from the user
    let mut input = [0u8; 1];
    stdin().read(&mut input).unwrap();
    r[c] = input[0] as u32;
    // print whatever is in r[c]
    //println!("{}", r[c]); // test code

    return 0;
}