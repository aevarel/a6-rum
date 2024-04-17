use::bitpack::*;

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
#[inline(always)]
pub fn regs_array(word: u32) -> [usize; 3] {
    // for further optimization, rewrite bitpack to use u32 instead of u64 to avoid casting
    let a = getu(word as u64, 3, 6).unwrap() as usize;
    let b = getu(word as u64, 3, 3).unwrap() as usize;
    let c = getu(word as u64, 3, 0).unwrap() as usize;

    [a, b, c]
}