use std::{collections::VecDeque, vec};
use chrono::{DateTime, Local};
use std::collections::HashMap;

pub struct WwvbEncoder;

impl WwvbEncoder {
    
    pub fn encode(date_time: DateTime<Local>) -> VecDeque<char> {
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
}