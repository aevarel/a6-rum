pub mod math;
pub mod io;
pub mod control;
pub mod memory;
pub mod word;


// write tests for the functions in the memory module as they're the most imperative to test. 
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
        r[5] = 6;
        // c = 5, b = 1, r[c] = segment of 6 words, gets stored in m[1]
        memory::map(&mut r, &mut m, &mut saved_ids, 205);
        expected_m.push(vec![0; 6]);
        assert_eq!(m, expected_m);

    }
    
    #[test]
        // check if the last segment was unmapped, in which the memory segment should be popped
    fn test_unmap_last() {
        let mut r = [0; 8];
        let mut m: Vec<Vec<u32>> = vec![Vec::new(); 1];
        let mut saved_ids: Vec<u32> = Vec::new();
        // map a segment with 4 words
        r[5] = 4;
        memory::map(&mut r, &mut m, &mut saved_ids, 205); // c = 5, b = 1, segment of 4 words gets stored in m[1]
        r[4] = 5;
        // map a segment with 5 words
        memory::map(&mut r, &mut m, &mut saved_ids, 212); // c = 4, b = 2, segment of 5 words gets stored in m[2]

        memory::unmap(&mut r, &mut m, &mut saved_ids, 2); // c = 2, m[2] gets popped off the stack 
        // assert that the segment with 5 words was indexed by 2 and has been popped off
        // since it was in last slot of memory
        assert_eq!(m.len(), 2);
        assert_eq!(m[0], vec![]);
    }

    #[test]
fn test_unmap_saved_ids() {
        let mut r = [0; 8];
        let mut m: Vec<Vec<u32>> = vec![Vec::new(); 1];
        let mut saved_ids: Vec<u32> = Vec::new();
        // map a segment with 4 words
        r[5] = 4;
        memory::map(&mut r, &mut m, &mut saved_ids, 205); // c = 5, b = 1, segment of 4 words gets stored in m[1]
        r[4] = 5;
        // map a segment with 5 words
        memory::map(&mut r, &mut m, &mut saved_ids, 212); // c = 4, b = 2, segment of 5 words gets stored in m[2]

        memory::unmap(&mut r, &mut m, &mut saved_ids, 1); // c = 1, m[1] gets emptied and marked for reuse 
        // assert that the segment with 5 words was indexed by 1 and has been emptied, but not popped
        assert_eq!(m.len(), 3);
        assert_eq!(m[1], vec![]);
        // check that its id was saved in saved_ids
        assert_eq!(saved_ids, [1]);
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
        assert_eq!(r[0], 1);
    }


    /// Tests for the store function
    #[test]
    // checks whether the sstore function maps the memory correctly and if it is correctly pushed onto the program's vector of instructions
    fn test_store() {
        let mut r = [0; 8];
        let mut m: Vec<Vec<u32>> = vec![Vec::new(); 1];
        let mut saved_ids: Vec<u32> = Vec::new();
        r[5] = 4;
        memory::map(&mut r, &mut m, &mut saved_ids, 205); // a = 3, b = 1, c = 5, r[c] = 4, words = 4
        r[5] = 4;
        r[3] = 1;
        r[1] = 1;
        memory::sstore(&mut r, &mut m, 205); // store m[]
        print!("{:?}", m);
        assert_eq!(m[1][1], 4);
    }
}


