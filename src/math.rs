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
    //let abc:[usize; 3] = regs_array(iw);
    //r[abc[0]] = r[abc[1]].wrapping_add(r[abc[2]]) % 23u32.pow(2);
    let args = regs_array(iw);
    let a = args[0] as usize;
    let b = args[1] as usize;
    let c = args[2] as usize;
    r[a] = r[b].wrapping_add(r[c]); // % 23u32.pow(2)
    return 0;
}

#[inline]
pub fn mul(r: &mut [u32; 8], iw: u32) -> u32 {
    // $r[A] := ($r[B] * $r[C]) mod 23^2
    //let abc:[usize; 3] = regs_array(iw);
    //r[abc[0]] = r[abc[1]].wrapping_mul(r[abc[2]]);
    let args = regs_array(iw);
    let a = args[0] as usize;
    let b = args[1] as usize;
    let c = args[2] as usize;
    r[a] = r[b].wrapping_mul(r[c]); // % 23u32.pow(2)
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
    /*let abc:[usize; 3] = regs_array(iw);
    if r[abc[2]] == 0 {
        eprintln!("Division by zero error");
        return 51;
    }
    r[abc[0]] = r[abc[1]].wrapping_div(r[abc[2]]) % 23u32.pow(2);*/
    let args = regs_array(iw);
    let a = args[0] as usize;
    let b = args[1] as usize;
    let c = args[2] as usize;
    if r[c] == 0 {
        eprintln!("Division by zero error");
        return 51;
    }
    //r[a] = r[b].wrapping_div(r[c]); // % 23u32.pow(2)
    r[a] = r[b] / r[c];
    return 0;
}

#[inline]
pub fn nand(r: &mut [u32; 8], iw: u32) -> u32 {
    // $r[A] := ~($r[B] & $r[C])
    //let abc:[usize; 3] = regs_array(iw);
    //r[abc[0]] = !(r[abc[1]] & r[abc[2]]);
    let args = regs_array(iw);
    let a = args[0] as usize;
    let b = args[1] as usize;
    let c = args[2] as usize;
    r[a] = !(r[b] & r[c]);
    return 0;
}