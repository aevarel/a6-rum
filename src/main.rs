use std::io::Read; 
use rum::math::*;
use rum::io::*;
use rum::control::*;
use rum::memory::*;
use rum::word::*;
use std::process;

fn main()  {

    // get single command line argument, which is the name of the program file
    let args: Vec<String> = std::env::args().collect();
    let input = Some(&args[1]);

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

    let mut raw_reader: Box<dyn std::io::BufRead> = match input {
        None => Box::new(std::io::BufReader::new(std::io::stdin())),
        Some(filename) => Box::new(std::io::BufReader::new(
            std::fs::File::open(filename).unwrap(),
        )),
    };

    let mut buf = Vec::<u8>::new();
    raw_reader.read_to_end(&mut buf).unwrap();
    
    let instructions: Vec<u32> = buf
        .chunks_exact(4)
        .map(|x| u32::from_be_bytes(x.try_into().unwrap()))
        .collect();

    m.push(instructions);

    // initialize the program counter
    let mut pc: i64 = 0;
    
    // loop until a halt instruction is reached
    let mut ret;
    loop {
    
        // fetch the instruction 
        let iw = m[0][pc as usize];

        let op = opcode(iw);

        pc += 1; // moved to here, maybe it'll work now?
        match op { 
            0 => ret = cmov(&mut r, iw),
            1 => ret = sload(&mut r, &mut m, iw),
            2 => ret = sstore(&mut r, &mut m, iw),
            3 => ret = add(&mut r, iw),
            4 => ret = mul(&mut r, iw),
            5 => ret = div(&mut r, iw),
            6 => ret = nand( &mut r, iw),
            7 => process::exit(0),
            8 => ret = map(&mut r, &mut m, &mut saved_ids, iw),
            9 => ret = unmap(&mut r, &mut m, &mut saved_ids, iw),
            10 => ret = out(&mut r, iw),
            11 => ret = inp(&mut r, iw),
            12 => ret = loadp(&mut r, &mut m, iw, &mut pc),
            13 => ret = loadv(&mut r, iw),
            _ => ret = op as u32 * 10,
        };        
        if ret != 0 {
            eprintln!("Error: {}", ret);
            process::exit(ret as i32);
        }
    }
}

