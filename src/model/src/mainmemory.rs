use serde_json::{Value, Map};
//use serde::{Deserialize, Serialize};
pub const MAX_SIZE: usize = 256;

#[derive(Debug, PartialEq)]
pub struct MainMemory {
    pub memory_space: [u8; MAX_SIZE],
}

impl MainMemory {
    pub fn new() -> MainMemory {
        MainMemory {
            memory_space: [0; MAX_SIZE],
        }
    }

    /// Function to reset memory to all 0s
    pub fn reset_memory(&mut self) {
        self.memory_space.fill(0);
    }

    /// Get a single byte at a certain address, does size check
    pub fn get_byte(&mut self, index: usize) -> Option<u8>{
        if index > MAX_SIZE-1 {
            return None;
        }
        return Some(self.memory_space[index]);
        
    }

    /// Sets a single byte at a certain address, does size check
    pub fn set_byte(&mut self, value: u8, index: usize) -> bool{
        if index > MAX_SIZE-1 {
            return false;
        }
        self.memory_space[index] = value;

        return true;
    }

    /// Loads a vec of bytes representing the program code into the main memory
    pub fn load_program(&mut self, mut bytes: Vec<u8>){
        let mut count = 0;
        for i in bytes.iter_mut() {
            self.memory_space[count] = *i;
            count += 1;
        }
    }

    /// Gets a quadword from memory starting at some address
    pub fn get_qword(&mut self, index: usize) -> Option<i64>{
        //If start of quadword is less than 8 bytes from the end of the memory space we cant do it
        if index > MAX_SIZE-9 {
            return None;
        }

        // Get those bytes as a slice and convert them to an i64 little endian
        let mut bytes: [u8; 8] = [0; 8];
        bytes.copy_from_slice(&self.memory_space[index .. index+8]);

        let num = i64::from_le_bytes(bytes);

        return Some(num);


    }
  
    /// Sets a quadword in memory
    pub fn set_qword(&mut self, value: i64, index: usize) -> bool{
        if index > MAX_SIZE-9 {
            return false;
        }

        // Convert quadword into little endian bytes, copy into memory space
        let bytes = value.to_le_bytes();

        self.memory_space[index .. index+8].copy_from_slice(&bytes);

        return true;
    }

    pub fn jsonify(&mut self, mode: usize) -> String {

        let mut map = Map::new();

        let str_memory_list: Vec<String>;

        //1 byte mode, e.g. byte mode
        if mode == 1{
            str_memory_list = self.memory_space.iter().map(|x|  format!("{:01$X}", x, 2)).collect();
        }
        //Alternatively, 8 byte mode
        else {
            str_memory_list = self.memory_space.chunks(8).map(|x| {
                let mut bytes: [u8; 8] = [0; 8];
                bytes.copy_from_slice(x);
                i64::from_le_bytes(bytes).to_string()
            }).collect();

        }

       
        map.insert("contents".to_string(), str_memory_list.into());

        let obj = Value::Object(map);

        return obj.to_string();


    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_byte_empty(){
        let mut main_memory = MainMemory::new();

        let actual_option = main_memory.get_byte(0);

        let expected: u8 = 0;

        match actual_option {
            None => panic!("Expected to get data"),
            Some(actual) => assert_eq!(actual, expected),
        }
    }

    #[test]
    fn test_get_middle_byte_empty(){
        let mut main_memory = MainMemory::new();

        let actual_option = main_memory.get_byte(128);

        let expected: u8 = 0;

        match actual_option {
            None => panic!("Expected to get data"),
            Some(actual) => assert_eq!(actual, expected),
        }
    }

    #[test]
    fn test_get_last_byte_empty(){
        let mut main_memory = MainMemory::new();

        let actual_option = main_memory.get_byte(MAX_SIZE-1);

        let expected: u8 = 0;

        match actual_option {
            None => panic!("Expected to get data"),
            Some(actual) => assert_eq!(actual, expected),
        }
    }

    #[test]
    fn test_get_byte_out_of_space_fails(){
        let mut main_memory = MainMemory::new();

        let actual_option = main_memory.get_byte(MAX_SIZE);

        match actual_option {
            None => {},
            Some(actual) => panic!("Expected to get None, not Some({:?})", actual),
        }
    }

    #[test]
    fn test_set_first_byte(){
        let mut main_memory = MainMemory::new();

        let success = main_memory.set_byte(15, 0);

        assert_eq!(success, true);

        let expected: u8 = 15;

        let actual_option = main_memory.get_byte(0);

        match actual_option {
            None => panic!("Expected to get data"),
            Some(actual) => assert_eq!(actual, expected),
        }
    }

    #[test]
    fn test_set_last_byte(){
        let mut main_memory = MainMemory::new();

        let success = main_memory.set_byte(15, MAX_SIZE-1);

        assert_eq!(success, true);

        let expected: u8 = 15;

        let actual_option = main_memory.get_byte(MAX_SIZE-1);

        match actual_option {
            None => panic!("Expected to get data"),
            Some(actual) => assert_eq!(actual, expected),
        }
    }

    #[test]
    fn test_set_out_of_bounds_fails(){
        let mut main_memory = MainMemory::new();

        let success = main_memory.set_byte(15, MAX_SIZE);

        assert_eq!(success, false);

    }


    #[test]
    fn test_get_first_qword_empty(){
        let mut main_memory = MainMemory::new();

        let actual_option = main_memory.get_qword(0);

        let expected: i64 = 0;

        match actual_option {
            None => panic!("Expected to get data"),
            Some(actual) => assert_eq!(actual, expected),
        }
    }

    #[test]
    fn test_get_middle_qword_empty(){
        let mut main_memory = MainMemory::new();

        let actual_option = main_memory.get_qword(128);

        let expected: i64 = 0;

        match actual_option {
            None => panic!("Expected to get data"),
            Some(actual) => assert_eq!(actual, expected),
        }
    }

    #[test]
    fn test_get_last_qword_empty(){
        let mut main_memory = MainMemory::new();

        let actual_option = main_memory.get_qword(MAX_SIZE-9);

        let expected: i64 = 0;

        match actual_option {
            None => panic!("Expected to get data"),
            Some(actual) => assert_eq!(actual, expected),
        }
    }

    #[test]
    fn test_get_qword_out_of_space_fails(){
        let mut main_memory = MainMemory::new();

        let actual_option = main_memory.get_qword(MAX_SIZE-8);

        match actual_option {
            None => {},
            Some(actual) => panic!("Expected to get None, not Some({:?})", actual),
        }
    }
    

    #[test]
    fn test_set_first_qword(){
        let mut main_memory = MainMemory::new();

        let success = main_memory.set_qword(1024, 0);

        assert_eq!(success, true);

        let expected: i64 = 1024;

        let actual_option = main_memory.get_qword(0);

        match actual_option {
            None => panic!("Expected to get data"),
            Some(actual) => assert_eq!(actual, expected),
        }
    }

    #[test]
    fn test_set_last_qword(){
        let mut main_memory = MainMemory::new();

        let success = main_memory.set_qword(1024, MAX_SIZE-9);

        assert_eq!(success, true);

        let expected: i64 = 1024;

        let actual_option = main_memory.get_qword(MAX_SIZE-9);

        match actual_option {
            None => panic!("Expected to get data"),
            Some(actual) => assert_eq!(actual, expected),
        }
    }

    #[test]
    fn test_set_qword_out_of_bounds_fails(){
        let mut main_memory = MainMemory::new();

        let success = main_memory.set_qword(1024, MAX_SIZE-8);

        assert_eq!(success, false);

    }

    #[test]
    fn test_set_qword_get_bytes_spreads(){
        let mut main_memory = MainMemory::new();

        let success = main_memory.set_qword(511, MAX_SIZE-9);

        assert_eq!(success, true);

        //Little endian?
        let expected1: u8 = 255;
        let expected2: u8 = 1;

        let actual1 = main_memory.get_byte(MAX_SIZE-9);
        let actual2 = main_memory.get_byte(MAX_SIZE-8);

        match actual1 {
            None => panic!("Expected to get data"),
            Some(actual) => assert_eq!(actual, expected1),
        }

        match actual2 {
            None => panic!("Expected to get data"),
            Some(actual) => assert_eq!(actual, expected2),
        }





    }



}