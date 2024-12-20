use std::{collections::VecDeque, vec};
use chrono::{DateTime, Local};
use std::collections::HashMap;

pub struct WwvbEncoder;

impl WwvbEncoder {
    
    pub fn encode(dateTime: DateTime<Local>) -> VecDeque<char> {
        let mut encoded_time = VecDeque::new();

        // Start of frame (1 bit)
        encoded_time.push_back('M');

        // Minutes


        encoded_time.push_back('H');
        encoded_time.push_back('H');    
        encoded_time.push_back('M');
        encoded_time.push_back('M');
        encoded_time.push_back('M');
        encoded_time.push_back('H');
        encoded_time.push_back('H');    



        return encoded_time;
    }

    

    fn get_bcd_map(num: u32) -> HashMap<u32, u8> {
        let mut bcd_map = HashMap::new();
    
        // Loop over each digit of the number
        let mut temp_num = num;
        let mut digit_index = 0;
        
        // Build a BCD map where the key is the digit, and the value is the BCD representation
        while temp_num > 0 {
            let digit = (temp_num % 10) as u32; // Extract the last decimal digit
            let bcd = Self::digit_to_bcd(digit);
            bcd_map.insert(digit, bcd);
            temp_num /= 10; // Remove the last decimal digit
        }
    
        bcd_map
    }
    
    // Convert a digit (0-9) to its BCD representation (4-bit binary)
    fn digit_to_bcd(digit: u32) -> u8 {
        match digit {
            0 => 0b0000,
            1 => 0b0001,
            2 => 0b0010,
            3 => 0b0011,
            4 => 0b0100,
            5 => 0b0101,
            6 => 0b0110,
            7 => 0b0111,
            8 => 0b1000,
            9 => 0b1001,
            _ => panic!("Invalid digit!"),
        }
    }
}