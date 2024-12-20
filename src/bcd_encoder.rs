use std::{fmt, mem};

type UBcdNumber = u32;

const BITS_PER_BYTE: usize = 8;
const BITS_PER_DIGIT: usize = 4;
const DIGITS_PER_BYTE: usize = BITS_PER_BYTE / BITS_PER_DIGIT;
const U_BCD_NUMBER_SIZE_IN_BYTES: usize = mem::size_of::<UBcdNumber>();
const U_BCD_NUMBER_MAX_REPRESENTABLE_DIGITS: usize = U_BCD_NUMBER_SIZE_IN_BYTES * DIGITS_PER_BYTE;

#[derive(Debug)]
pub enum BcdError {
    NumberTooLarge(UBcdNumber),
}
impl fmt::Display for BcdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BcdError::NumberTooLarge(number) => write!(f, "Number is too large for return variable size: {}", number),
        }
    }
}

impl std::error::Error for BcdError {}


pub fn binary_to_bcd(binary_number: u32) -> Result<UBcdNumber, BcdError> {
    let mut bcd_accumulator: UBcdNumber = 0;

    let mut number_extractor = binary_number;
    let mut digit_index = 0;
    
    // Build a BCD map where the key is the digit, and the value is the BCD representation
    while number_extractor > 0 {
        if digit_index >= U_BCD_NUMBER_MAX_REPRESENTABLE_DIGITS {
            return Err(BcdError::NumberTooLarge(binary_number));
        }

        // Extract the last decimal digit
        let digit = number_extractor % 10; 
        let bcd_digit = digit_to_bcd(digit);

        // Add digit to accumulator
        bcd_accumulator |= bcd_digit << (BITS_PER_DIGIT * digit_index);

        // Remove the last decimal digit
        number_extractor /= 10;

        // Post increment
        digit_index +=1;
    }

    Ok(bcd_accumulator)
}

// Convert a digit (0-9) to its BCD representation (4-bit binary)
fn digit_to_bcd(digit: u32) -> u32 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_to_bcd_basic() {
        // Test BCD conversion for small numbers
        assert_eq!(binary_to_bcd(0).unwrap(),   0b0000);
        assert_eq!(binary_to_bcd(5).unwrap(),   0b0101);
        assert_eq!(binary_to_bcd(123).unwrap(), 0b000100100011); // BCD of 123
        assert_eq!(binary_to_bcd(456).unwrap(), 0b010001010110); // BCD of 456
    }

    #[test]
    fn test_binary_to_bcd_large_number() {
        // Test large numbers and check that the error is returned when the number is too large
        let result = binary_to_bcd(1234567890); // 10-digit number, too large for a u32 BCD

        match result {
            Err(BcdError::NumberTooLarge(n)) => assert_eq!(n, 1234567890),
            Ok(_) => panic!("Expected an error, but got Ok!"),
        }
    }


    #[test]
    fn test_digit_to_bcd() {
        // Ensure each digit returns the correct 4-bit BCD representation
        assert_eq!(digit_to_bcd(0), 0b0000 as UBcdNumber);
        assert_eq!(digit_to_bcd(1), 0b0001 as UBcdNumber);
        assert_eq!(digit_to_bcd(9), 0b1001 as UBcdNumber);
        assert_eq!(digit_to_bcd(5), 0b0101 as UBcdNumber);
    }
}