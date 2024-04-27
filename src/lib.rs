pub mod math;
pub mod io;
pub mod control;
pub mod memory;
pub mod word;


// write tests for the functions in the memory module
#[cfg(test)]
mod tests {
    use super::*;

    /// Tests for the map function
    #[test]
    // checks whether the sload function maps the memory correctly and if it is correctly pushed onto the program's vector of instructions
    fn test_map() {
        let mut r = [0; 8];
        let mut m: Vec<Vec<u32>> = vec![Vec::new(); 1];
        let mut expected_m = m.clone();
        let mut saved_ids: Vec<u32> = Vec::new();
        memory::map(&mut r, &mut m, &mut saved_ids, 3584); // map r[0] = 0, r[1] = 0, r[2] = 0, words = 7
        expected_m.push(vec![0; 7]);
        assert_eq!(m, expected_m);
    }
    #[test]
    fn test_map_updates_register() {
        let mut r = [0; 8];
        let mut m: Vec<Vec<u32>> = vec![Vec::new(); 1];
        let mut saved_ids: Vec<u32> = Vec::new();
        memory::map(&mut r, &mut m, &mut saved_ids, 3584); // map r[0] = 0, r[1] = 0, r[2] = 0, words = 7

        // Check if r[2] was updated with the index of the newly mapped segment
        assert_eq!(r[2], 1);
    }

    /// Tests for the unmap function
    #[test]
    // checks whether the sload maps the memory and if it is correctly unmapped 
    fn test_unmap() {
        let mut r = [0; 8];
        let mut m: Vec<Vec<u32>> = vec![Vec::new(); 1];
        let expected_m = m.clone();
        let mut saved_ids: Vec<u32> = Vec::new();
        memory::map(&mut r, &mut m, &mut saved_ids, 3584); // map r[0] = 0, r[1] = 0, r[2] = 0, words = 7
        memory::unmap(&mut r, &mut m, &mut saved_ids, 3584); // unmap r[0] = 0, r[1] = 0, r[2] = 0
        assert_eq!(m, expected_m);
    }
    #[test]
    fn test_unmap_updates_register() {
        let mut r = [0; 8];
        let mut m: Vec<Vec<u32>> = vec![Vec::new(); 1];
        let mut saved_ids: Vec<u32> = Vec::new();
        memory::map(&mut r, &mut m, &mut saved_ids, 3584); // map r[0] = 0, r[1] = 0, r[2] = 0, words = 7

        // Check if r[2] was updated with the index of the newly mapped segment
        assert_eq!(r[2], 1);

        memory::unmap(&mut r, &mut m, &mut saved_ids, 3584); // unmap r[0] = 0, r[1] = 0, r[2] = 0

        // Check if r[2] was updated with 0
        assert_eq!(r[2], 0);
    }

    /// Tests for the load function
    #[test]
    // checks whether the sload function maps the memory correctly and if it is correctly pushed onto the program's vector of instructions
    fn test_load() {
        let mut r = [0; 8];
        let mut m: Vec<Vec<u32>> = vec![Vec::new(); 1];
        let mut saved_ids: Vec<u32> = Vec::new();
        memory::map(&mut r, &mut m, &mut saved_ids, 3584); // map r[0] = 0, r[1] = 0, r[2] = 0, words = 7
        memory::sload(&mut r, &mut m, 3584); // load r[0] = 0, r[1] = 0, r[2] = 0, words = 7
        assert_eq!(r[0], 0);
    }
    /// Tests for the store function
    #[test]
    // checks whether the sload function maps the memory correctly and if it is correctly pushed onto the program's vector of instructions
    fn test_store() {
        let mut r = [0; 8];
        let mut m: Vec<Vec<u32>> = vec![Vec::new(); 1];
        let mut saved_ids: Vec<u32> = Vec::new();
        memory::map(&mut r, &mut m, &mut saved_ids, 3584); // map r[0] = 0, r[1] = 0, r[2] = 0, words = 7
        memory::sstore(&mut r, &mut m, 3584); // store r[0] = 0, r[1] = 0, r[2] = 0, words = 7
        assert_eq!(m[1][0], 0);
    }
}


