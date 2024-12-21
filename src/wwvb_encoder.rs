use std::{collections::VecDeque, vec};
use chrono::{Date, DateTime, Datelike, Local, NaiveDate, Timelike};
use std::collections::HashMap;

use crate::bcd_encoder::{self, UBcdNumber};

pub struct WwvbEncoder;

const BIT_TO_SYMBOL: [char; 2] = ['L','H'];

impl WwvbEncoder {
    
    pub fn encode(date_time: DateTime<Local>) -> VecDeque<char> {
        let mut encoded_time = VecDeque::new();

        // Start of frame (1 bit)
        encoded_time.push_back('M');

        // Minutes
        encoded_time.append(&mut Self::get_minutes(date_time.minute()));

        // Marker
        encoded_time.push_back('M');

        // Unused (x2)
        encoded_time.push_back('L');
        encoded_time.push_back('L');

        // Hours
        encoded_time.append(&mut Self::get_hours(date_time.hour()));

        // Marker
        encoded_time.push_back('M');

        // Unused (x2)
        encoded_time.push_back('L');
        encoded_time.push_back('L');

        // Day of year
        encoded_time.append(&mut Self::get_day_of_year(date_time.ordinal()));

        // Unused (x2)
        encoded_time.push_back('L');
        encoded_time.push_back('L');

        // DUT1 sign
        encoded_time.append(&mut Self::get_dut1_sign());

        // Marker
        encoded_time.push_back('M');

        // DUT1 value
        encoded_time.append(&mut &mut Self::get_dut1_value());

        // Unused
        encoded_time.push_back('L');

        // Year
        encoded_time.append(&mut Self::get_year(date_time.year()));

        // Unused
        encoded_time.push_back('L');

        // Leap year indicator
        encoded_time.append(&mut Self::get_leap_year_indicator(date_time.year()));

        // Leap second at end of month
        encoded_time.append(&mut Self::get_leap_second_at_end_of_month());

        // DST status value
        encoded_time.append(&mut Self::get_dst_status_value(&date_time));

        // Marker
        encoded_time.push_back('M');

        return encoded_time;
    }

    fn get_symbol(bcd: &UBcdNumber, mask: UBcdNumber) -> char {
        BIT_TO_SYMBOL[((bcd & mask) != 0) as usize]
    }

    fn get_minutes(minutes: u32) -> VecDeque<char> {
        let mut encoded_minutes = VecDeque::new();

        let bcd_minutes = bcd_encoder::binary_to_bcd(minutes).unwrap();

        encoded_minutes.push_back(Self::get_symbol(&bcd_minutes, bcd_encoder::BCD_40_MASK));
        encoded_minutes.push_back(Self::get_symbol(&bcd_minutes, bcd_encoder::BCD_20_MASK));
        encoded_minutes.push_back(Self::get_symbol(&bcd_minutes, bcd_encoder::BCD_10_MASK));

        encoded_minutes.push_back('L');

        encoded_minutes.push_back(Self::get_symbol(&bcd_minutes, bcd_encoder::BCD_8_MASK));
        encoded_minutes.push_back(Self::get_symbol(&bcd_minutes, bcd_encoder::BCD_4_MASK));
        encoded_minutes.push_back(Self::get_symbol(&bcd_minutes, bcd_encoder::BCD_2_MASK));
        encoded_minutes.push_back(Self::get_symbol(&bcd_minutes, bcd_encoder::BCD_1_MASK));
        
        encoded_minutes
    }

    fn get_hours(hours: u32) -> VecDeque<char> {
        let mut encoded_hours = VecDeque::new();

        let bcd_hours = bcd_encoder::binary_to_bcd(hours).unwrap();

        encoded_hours.push_back(Self::get_symbol(&bcd_hours, bcd_encoder::BCD_20_MASK));
        encoded_hours.push_back(Self::get_symbol(&bcd_hours, bcd_encoder::BCD_10_MASK));

        encoded_hours.push_back('L');

        encoded_hours.push_back(Self::get_symbol(&bcd_hours, bcd_encoder::BCD_8_MASK));
        encoded_hours.push_back(Self::get_symbol(&bcd_hours, bcd_encoder::BCD_4_MASK));
        encoded_hours.push_back(Self::get_symbol(&bcd_hours, bcd_encoder::BCD_2_MASK));
        encoded_hours.push_back(Self::get_symbol(&bcd_hours, bcd_encoder::BCD_1_MASK));
        
        encoded_hours
    }

    fn get_day_of_year(day_of_year: u32) -> VecDeque<char> {
        let mut encoded_day_of_year = VecDeque::new();

        let bcd_day_of_year = bcd_encoder::binary_to_bcd(day_of_year).unwrap();

        encoded_day_of_year.push_back(Self::get_symbol(&bcd_day_of_year, bcd_encoder::BCD_200_MASK));
        encoded_day_of_year.push_back(Self::get_symbol(&bcd_day_of_year, bcd_encoder::BCD_100_MASK));
        encoded_day_of_year.push_back(Self::get_symbol(&bcd_day_of_year, bcd_encoder::BCD_80_MASK));

        encoded_day_of_year.push_back('L');

        encoded_day_of_year.push_back(Self::get_symbol(&bcd_day_of_year, bcd_encoder::BCD_40_MASK));
        encoded_day_of_year.push_back(Self::get_symbol(&bcd_day_of_year, bcd_encoder::BCD_20_MASK));
        encoded_day_of_year.push_back(Self::get_symbol(&bcd_day_of_year, bcd_encoder::BCD_10_MASK));

        encoded_day_of_year.push_back('M');

        encoded_day_of_year.push_back(Self::get_symbol(&bcd_day_of_year, bcd_encoder::BCD_8_MASK));
        encoded_day_of_year.push_back(Self::get_symbol(&bcd_day_of_year, bcd_encoder::BCD_4_MASK));
        encoded_day_of_year.push_back(Self::get_symbol(&bcd_day_of_year, bcd_encoder::BCD_2_MASK));
        encoded_day_of_year.push_back(Self::get_symbol(&bcd_day_of_year, bcd_encoder::BCD_1_MASK));
        
        encoded_day_of_year
    }

    // TODO: hardcoded to return +
    fn get_dut1_sign() -> VecDeque<char> {
        let mut encoded_dut1_sign = VecDeque::new();

        encoded_dut1_sign.push_back('H');
        encoded_dut1_sign.push_back('L');
        encoded_dut1_sign.push_back('H');

        encoded_dut1_sign
    }

    // TODO: hardcoded to return 0
    fn get_dut1_value() -> VecDeque<char> {
        let mut encoded_dut1_value = VecDeque::new();

        encoded_dut1_value.push_back('L');
        encoded_dut1_value.push_back('L');
        encoded_dut1_value.push_back('L');
        encoded_dut1_value.push_back('L');

        encoded_dut1_value
    }

    fn get_year(year: i32) -> VecDeque<char> {
        let mut encoded_year = VecDeque::new();

        let bcd_year = bcd_encoder::binary_to_bcd(year as u32).unwrap();

        encoded_year.push_back(Self::get_symbol(&bcd_year, bcd_encoder::BCD_80_MASK));
        encoded_year.push_back(Self::get_symbol(&bcd_year, bcd_encoder::BCD_40_MASK));
        encoded_year.push_back(Self::get_symbol(&bcd_year, bcd_encoder::BCD_20_MASK));
        encoded_year.push_back(Self::get_symbol(&bcd_year, bcd_encoder::BCD_10_MASK));

        encoded_year.push_back('M');

        encoded_year.push_back(Self::get_symbol(&bcd_year, bcd_encoder::BCD_8_MASK));
        encoded_year.push_back(Self::get_symbol(&bcd_year, bcd_encoder::BCD_4_MASK));
        encoded_year.push_back(Self::get_symbol(&bcd_year, bcd_encoder::BCD_2_MASK));
        encoded_year.push_back(Self::get_symbol(&bcd_year, bcd_encoder::BCD_1_MASK));

        encoded_year
    }

    fn get_leap_year_indicator(year: i32) -> VecDeque<char> {
        let mut encoded_leap_year = VecDeque::new();

        let is_leap = NaiveDate::from_ymd_opt(year, 1, 1).unwrap().leap_year();
        encoded_leap_year.push_back(BIT_TO_SYMBOL[is_leap as usize]);

        encoded_leap_year
    }

    // TODO: Currently placeholder
    fn get_leap_second_at_end_of_month() -> VecDeque<char> {
        let mut encoded_leap_second = VecDeque::new();

        encoded_leap_second.push_back('L');

        encoded_leap_second
    }

    // TODO: Currently placeholder
    fn get_dst_status_value(date_time: &DateTime<Local>) -> VecDeque<char>{
        let mut encoded_dst = VecDeque::new();

        encoded_dst.push_back('L');
        encoded_dst.push_back('L');

        encoded_dst
    }
}