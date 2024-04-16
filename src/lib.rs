use::bitpack::*;
pub mod math;
pub mod io;
pub mod control;
pub mod memory;

/// Extracts the opcode from an instruction word
/// 
/// # Arguments
/// 
/// * `word` - the instruction word
/// 
/// # Returns
/// 
/// The opcode
/// 
/// # Panics
/// 
/// If the opcode is not a valid value
#[inline]
//pub fn opcode(word: u32) -> u8 {  } 


/// Extracts the registers from an instruction word
/// 
/// # Arguments
/// 
/// * `word` - the instruction word
/// 
/// # Returns
/// 
/// An array of 3 u8s representing the registers
#[inline]
pub fn regs_array(word: u32) -> [u8; 3] {
    let a = getu(word as u64, 3, 0).unwrap() as u8;
    let b = getu(word as u64, 3, 3).unwrap() as u8;
    let c = getu(word as u64, 3, 6).unwrap() as u8;

    [a, b, c]
}