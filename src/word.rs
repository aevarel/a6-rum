//use core::panic;
use::bitpack::bitpack::*;


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
    getu(word as u64, 4, 28).unwrap() as u8
} 


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
    getu(word as u64, 3, 25).unwrap() as usize
}











/* 
#[derive(Debug)]
pub struct instruct_word {
    word: u32,
    opcode: u8,
    registers: [usize; 3],
    value: Option<u32>,
}
/// Implementation of the instruct_word struct
/// Initializes with the opcode from the 4 most significant bits of the instruction word,
/// the registers from the 9 least significant bits of the word,
/// and the word itself in case it is needed for debugging or other usage
/// 
/// if the opcode is 13, the value is extracted from the 25 least significant bits of the word,
/// and the b and c registers are set to 14 without manipulating the value of the word, as it is not needed.
/// 
/// # Arguments
/// 
/// * `word` - the instruction word
/// 
/// # Returns
/// 
/// An instruct_word struct
/// 
/// # Panics
/// 
/// If the opcode is not a valid value
/// 
/// 
impl instruct_word {
    pub fn from_word(word: u32) -> Self {
        let word: u32 = word;
        if getu(word as u64, 4, 28).unwrap() > 13 {
            // This should never happen
            panic!("Error 140");
        }
        // Extract instruction word as parts
        let opcode = getu(word as u64, 4, 28).unwrap() as u8;
        let (a, b, c, value) = if opcode == 13 {
            // Load instruction, register b,c set to 14 (non-existent registers), value extracted from 25 lsb of word
            (getu(word as u64, 3, 25),14, 14, Some(getu(word as u64, 25, 0).unwrap() as u32))
        } else {
            // All other instructions
            let a = getu(word as u64, 3, 6).unwrap() as usize;
            let b = getu(word as u64, 3, 3).unwrap() as usize;
            let c = getu(word as u64, 3, 0).unwrap() as usize;
            (a,b, c, None)
        };
        Self {
            word,
            opcode,
            registers: [a, b, c],
            value,
        }
    }
}
*/
