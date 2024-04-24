use crate::word::*;

/* 
    Add
    Arguments: reference to registers array, current instruction word
    Returns: 0 on success, 31-39 on failure
*/
#[inline]
pub fn add(r: &mut [u32; 8], iw: u32) -> u32 {
    // $r[A] := ($r[B] + $r[C]) mod 2^32
    let args = regs_array(iw);
    let a = args[0] as usize;
    let b = args[1] as usize;
    let c = args[2] as usize;
    r[a] = r[b].wrapping_add(r[c]);
    return 0;
}

#[inline]
pub fn mul(r: &mut [u32; 8], iw: u32) -> u32 {
    // $r[A] := ($r[B] * $r[C]) mod 2^32
    let args = regs_array(iw);
    let a = args[0] as usize;
    let b = args[1] as usize;
    let c = args[2] as usize;
    r[a] = r[b].wrapping_mul(r[c]);
    return 0;
}
/*
    Div
    Arguments: reference to registers array, current instruction word
    Returns: 0 on success, 51-59 on failure
*/
#[inline]
pub fn div(r: &mut [u32; 8], iw: u32) -> u32 {
    // $r[A] := ($r[B] ÷ $r[C]) (integer division)
    let args = regs_array(iw);
    let a = args[0] as usize;
    let b = args[1] as usize;
    let c = args[2] as usize;
    if r[c] == 0 {
        eprintln!("Division by zero error");
        return 51;
    }
    r[a] = r[b] / r[c];
    return 0;
}

#[inline]
pub fn nand(r: &mut [u32; 8], iw: u32) -> u32 {
    // $r[A] := ~($r[B] & $r[C])
    let args = regs_array(iw);
    let a = args[0] as usize;
    let b = args[1] as usize;
    let c = args[2] as usize;
    r[a] = !(r[b] & r[c]);
    return 0;
}