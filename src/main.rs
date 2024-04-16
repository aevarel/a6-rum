use::bitpack::*;
use rum::math::*;
use rum::io::*;
use rum::control::*;
use rum::memory::*;

fn main() {

    // get single command line argument, which is the name of the program file
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    // array of u32s of length 8 to store virtual registers
    let mut registers: [u32; 8] = [0; 8];

    // vector of vectors of u32s to store instructions
    let mut instructions: Vec<Vec<u32>> = Vec::new();
    // index 0 is the program from the file
    //instructions.push(read_program(filename));

    // program counter
    let mut pc: u32 = 0;

    // loop until a halt instruction is reached
    

}
