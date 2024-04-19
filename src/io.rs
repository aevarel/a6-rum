use std::io::{stdin, stdout, Write};
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
    print!("{}", r[c]);
    stdout().flush().unwrap();

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
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    r[c] = input.chars().next().unwrap() as u32;

    return 0;
}