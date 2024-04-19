use std::io::Read; 
use rum::math::*;
use rum::io::*;
use rum::control::*;
use rum::memory::*;
use rum::word::*;

fn main()  {

    // get single command line argument, which is the name of the program file
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    // array of u32s of length 8 to store virtual registers
    let mut r: [u32; 8] = [0; 8];

    // vector of vectors of u32s to store memory
    let mut m: Vec<Vec<u32>> = Vec::new();

    // saved segment identifiers
    let mut saved_ids: Vec<u32> = Vec::new();

    /*
        The command line argument is the name of the program file.
        This is a binary file, which is read as a stream of u32s.
        The u32s are stored inside the inner vector m[0].
    */

    // open the file
    let file = std::fs::File::open(filename).unwrap();

    let mut reader = std::io::BufReader::new(file);
    let mut buffer = [0; 4];
    if m.is_empty() {
        m.push(Vec::new());
    }
    
    // read the file as a stream of u32s
    while let Ok(_) = reader.read_exact(&mut buffer) {
        let word = u32::from_le_bytes(buffer);
        m[0].push(word);
        // println!("{:032b}", word); // test code
    }


    // initialize the program counter
    let mut pc: u32 = 0;
    

    // loop until a halt instruction is reached

    loop {
        // fetch the instruction 
        let iw = m[0][pc as usize];
        let op = opcode(iw);
        match op { 
            0 => cmov(&mut r, iw),
            1 => sload(&mut r, &mut m, iw),
            2 => sstore(&mut r, &mut m, iw),
            3 => add(&mut r, iw),
            4 => mul(&mut r, iw),
            5 => div(&mut r, iw),
            6 => nand( &mut r, iw),
            7 => break,
            8 => map(&mut r, &mut m, &mut saved_ids, iw),
            9 => unmap(&mut r, &mut m, &mut saved_ids, iw),
            10 => out(&mut r, iw),
            11 => inp(&mut r, iw),
            12 => loadp(&mut r, &mut m, iw, &mut pc),
            13 => loadv(&mut r, iw),
            _ => std::process::exit(140),
        };        
    }
}

