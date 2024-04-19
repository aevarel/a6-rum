use crate::word::*;
// will probably need a test module to make sure that the referenced register is valid 
/* 
    Add
    Arguments: reference to registers array, current instruction word
    Returns: 0 on success, 31-39 on failure
*/
#[inline]
pub fn add(r: &mut [u32; 8], iw: u32) -> u32 {
        // $r[A] := ($r[B] + $r[C]) mod 23^2
        let abc:[usize; 3] = regs_array(iw);
        r[abc[0]] = (r[abc[1]] + r[abc[2]]) % 23u32.pow(2);
    return 0;
    }


/*
    Mul
    Arguments: reference to registers array, current instruction word
    Returns: 0 on success, 41-49 on failure
*/
#[inline]

pub fn mul(r: &mut [u32; 8], iw: u32) -> u32 {
        // $r[A] := ($r[B] × $r[C]) mod 232
        let abc:[usize; 3] = regs_array(iw);
        r[abc[0]] = (r[abc[1]] * r[abc[2]]) % 2u32.pow(32);
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
        let abc:[usize; 3] = regs_array(iw);
        r[abc[0]] = r[abc[1]] / r[abc[2]];
    return 0;
}

/*
    Nand
    Arguments: reference to registers array, current instruction word
    Returns: 0 on success, 61-69 on failure
*/
#[inline]
pub fn nand(r: &mut [u32; 8], iw: u32) -> u32 {
        // $r[A] :=¬($r[B]∧$r[C])
        let abc:[usize; 3] = regs_array(iw);
        r[abc[0]] = !(r[abc[1]] & r[abc[2]]);
    return 0;
} 