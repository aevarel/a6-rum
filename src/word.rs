use::bitpack::*;

/// Extracts the opcode from the 4 MSB a 32-bit instruction word
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
    bitpack::getu(word as u64, 4, 28).unwrap() as u8
} 


/// Extracts the registers from an instruction word
/// 
/// # Arguments
/// 
/// * `word` - the instruction word
/// 
/// # Returns
/// 
/// An array of 3 usizes representing the registers

#[inline(always)]
pub fn regs_array(word: u32) -> [usize; 3] {
    // for further optimization, rewrite bitpack to use u32 instead of u64 to avoid casting
    let a = bitpack::getu(word as u64, 3, 6).unwrap() as usize;
    let b = bitpack::getu(word as u64, 3, 3).unwrap() as usize;
    let c = bitpack::getu(word as u64, 3, 0).unwrap() as usize;

    [a, b, c]
}

/// In the case of the 'load' instruction, extract the register 
/// from the 3 most significant bits after the opcode bits 
/// 
/// # Arguments
/// 
/// * `word` - the instruction word
/// 
/// # Returns
/// 
/// The register
#[inline(always)]
pub fn load_reg(word: u32) -> usize {
    bitpack::getu(word as u64, 3, 25).unwrap() as usize
}
