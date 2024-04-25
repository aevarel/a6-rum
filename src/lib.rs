pub mod math;
pub mod io;
pub mod control;
pub mod memory;
pub mod word;


// write tests for the functions in the memory module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let mut r = [0; 8];
        let mut m: Vec<Vec<u32>> = vec![Vec::new(); 1];
        let mut expected_m = m.clone();
        let mut saved_ids: Vec<u32> = Vec::new();
        memory::map(&mut r, &mut m, &mut saved_ids, 3584); // map r[0] = 0, r[1] = 0, r[2] = 0, words = 7
        expected_m.push(vec![0; 7]);
        assert_eq!(m, expected_m);
    }
}
