use::bitpack::*;

/*
    Map: map/allocate new memory segment
    Arguments: pointer to registers, pointer to memory, pointer to saved segment identifiers, instruction word
    Returns: 0 on success, 81-89 on failure
*/
#[inline]
pub fn map(r: &mut [u32; 32], m: &mut [u32; 1024], saved_ids: &mut [u32; 32], iw: u32) -> u32 {
    
    return 0;
}

/* 
    Unmap: unmap/deallocate/free memory segment
    Arguments: pointer to registers, pointer to memory, pointer to saved segment identifiers, instruction word
    Returns: 0 on success, 91-99 on failure
*/
#[inline]
pub fn unmap(r: &mut [u32; 32], m: &mut [u32; 1024], saved_ids: &mut [u32; 32], iw: u32) -> u32 {
    

    return 0;
}

/*
    Sload: segment load
    Arguments: pointer to registers, pointer to memory, instruction word
    Returns: 0 on success, 11-19 on failure
*/
#[inline]
pub fn sload(r: &mut [u32; 32], m: &mut [u32; 1024], iw: u32) -> u32 {
    
    return 0;
}

/*
    Sstore: segment store
    Arguments: pointer to registers, pointer to memory, instruction word
    Returns: 0 on success, 21-29 on failure
*/
#[inline]
pub fn sstore(r: &mut [u32; 32], m: &mut [u32; 1024], iw: u32) -> u32 {
    
    return 0;
}

/*
    Loadp: load program
    Arguments: pointer to registers, pointer to memory, pointer to program counter, instruction word
    Returns: 0 on success, 121-129 on failure
*/
#[inline]
pub fn loadp(r: &mut [u32; 32], m: &mut [u32; 1024], pc: &mut u32, iw: u32) -> u32 {
    
    return 0;
}

/*
    Loadv: load value
    Arguments: pointer to registers, pointer to memory, instruction word
    Returns: 0 on success, 131-139 on failure
*/
#[inline]
pub fn loadv(r: &mut [u32; 32], m: &mut [u32; 1024], iw: u32) -> u32 {
    
    return 0;
}