use::bitpack::bitpack::*;


/// Extracts the opcode from the 4 MSB of a 32-bit instruction word
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
pub fn opcode(word: u32) -> u8{
    getu(word as u64, 4, 28).unwrap() as u8
} 

/// Extracts the registers from an instruction word of 32-bits,
/// where a, b, and c are the nine least significant bits of the word,
/// represented by three groups of three bits each of usize type.
/// 
/// # Arguments
/// 
/// * `word` - the instruction word in 'u32' format
/// 
/// # Returns
/// 
/// An array of 3 usize values representing the registers, with max width of 3 bits each

#[inline(always)]
pub fn regs_array(word: u32) -> [usize; 3] {
    // for further optimization, rewrite bitpack to use u32 instead of u64 to avoid casting
    let a = getu(word as u64, 3, 6).unwrap() as usize;
    let b = getu(word as u64, 3, 3).unwrap() as usize;
    let c = getu(word as u64, 3, 0).unwrap() as usize;

    [a, b, c]
}
