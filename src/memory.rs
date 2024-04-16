use::bitpack::*;

/*
    Map: map/allocate new memory segment
    Arguments: pointer to registers, pointer to memory, pointer to saved segment identifiers, instruction word
    Returns: 0 on success, 81-89 on failure
*/
pub fn map(r: &mut [u32; 32], m: &mut [u32; 1024], saved_ids: &mut [u32; 32], iw: u32) -> u32 {
    
    return 0;
}

/* 
    Unmap: unmap/deallocate/free memory segment
    Arguments: pointer to registers, pointer to memory, pointer to saved segment identifiers, instruction word
    Returns: 0 on success, 91-99 on failure
*/
pub fn unmap(r: &mut [u32; 32], m: &mut [u32; 1024], saved_ids: &mut [u32; 32], iw: u32) -> u32 {
    
    return 0;
}

/*
    Sload: segment load
    Arguments: pointer to registers, pointer to memory, instruction word
    Returns: 0 on success, 11-19 on failure
*/
pub fn sload(r: &mut [u32; 32], m: &mut [u32; 1024], iw: u32) -> u32 {
    
    return 0;
}

/*
    Sstore: segment store
    Arguments: pointer to registers, pointer to memory, instruction word
    Returns: 0 on success, 21-29 on failure
*/
pub fn sstore(r: &mut [u32; 32], m: &mut [u32; 1024], iw: u32) -> u32 {
    
    return 0;
}