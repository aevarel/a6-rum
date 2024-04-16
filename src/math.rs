use::bitpack::*;

// will probably need a test module to make sure that the referenced register is valid 
/* 
    Add
    Arguments: reference to registers array, current instruction word
    Returns: 0 on success, 31-39 on failure
*/
pub fn add(r: &mut [u32; 8], iw: u32) -> u32 {
        // $r[A] := ($r[B] + $r[C]) mod 23^2
        
    return 0;
}


/*
    Mul
    Arguments: reference to registers array, current instruction word
    Returns: 0 on success, 41-49 on failure
*/
pub fn mul(r: &mut [u32; 8], iw: u32) -> u32 {
        // $r[A] := ($r[B] × $r[C]) mod 232
    return 0;
}

/*
    Div
    Arguments: reference to registers array, current instruction word
    Returns: 0 on success, 51-59 on failure
*/
pub fn div(r: &mut [u32; 8], iw: u32) -> u32 {
        // $r[A] := ($r[B] ÷ $r[C]) (integer division)
    return 0;
}

/*
    Nand
    Arguments: reference to registers array, current instruction word
    Returns: 0 on success, 61-69 on failure
*/
pub fn nand(r: &mut [u32; 8], iw: u32) -> u32 {
        // $r[A] :=¬($r[B]∧$r[C])
    return 0;
}