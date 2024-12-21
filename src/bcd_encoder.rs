use std::{fmt, mem};

pub type UBcdNumber = u32;

pub const BCD_200_MASK:     UBcdNumber = 1 << 9;
pub const BCD_100_MASK:     UBcdNumber = 1 << 8;
pub const BCD_80_MASK:      UBcdNumber = 1 << 7;
pub const BCD_40_MASK:      UBcdNumber = 1 << 6;
pub const BCD_20_MASK:      UBcdNumber = 1 << 5;
pub const BCD_10_MASK:      UBcdNumber = 1 << 4;
pub const BCD_8_MASK:       UBcdNumber = 1 << 3;
pub const BCD_4_MASK:       UBcdNumber = 1 << 2;
pub const BCD_2_MASK:       UBcdNumber = 1 << 1;
pub const BCD_1_MASK:       UBcdNumber = 1 << 0;

const fn power_of_10(n: usize) -> UBcdNumber {
    if n == 0 {
        1
    } else {
        10 * power_of_10(n - 1)
    }
}

const BITS_PER_BYTE: usize = 8;
const BITS_PER_DIGIT: usize = 4;
const DIGITS_PER_BYTE: usize = BITS_PER_BYTE / BITS_PER_DIGIT;
const U_BCD_NUMBER_SIZE_IN_BYTES: usize = mem::size_of::<UBcdNumber>();
const U_BCD_NUMBER_MAX_REPRESENTABLE_DIGITS: usize = U_BCD_NUMBER_SIZE_IN_BYTES * DIGITS_PER_BYTE;
const MAX_REPRESENTABLE_BCD_NUMBER: UBcdNumber = (power_of_10(U_BCD_NUMBER_MAX_REPRESENTABLE_DIGITS) - 1) as UBcdNumber;

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

    if binary_number > MAX_REPRESENTABLE_BCD_NUMBER {
        return Err(BcdError::NumberTooLarge(binary_number));
    }
    
    // Build a BCD map where the key is the digit, and the value is the BCD representation
    while number_extractor > 0 {
        // Extract the last decimal digit
        let digit = number_extractor % 10; 

        // Add digit to accumulator
        bcd_accumulator |= digit << (BITS_PER_DIGIT * digit_index);

        // Remove the last decimal digit
        number_extractor /= 10;

        // Post increment
        digit_index +=1;
    }

    Ok(bcd_accumulator)
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
    fn test_binary_to_bcd_number_too_large() {
        // Test large numbers and check that the error is returned when the number is too large
        const NUMBER_TOO_LARGE: UBcdNumber = MAX_REPRESENTABLE_BCD_NUMBER + 1;
        let result = binary_to_bcd(NUMBER_TOO_LARGE);

        match result {
            Err(BcdError::NumberTooLarge(n)) => assert_eq!(n, NUMBER_TOO_LARGE),
            Ok(_) => panic!("Expected an error, but got Ok!"),
        }
    }

    #[test]
    fn test_binary_to_bcd_number_not_too_large() {
        // Test large numbers and check that no errors are returned when the number is large, but not too large
        const NUMBER_NOT_TOO_LARGE: UBcdNumber = MAX_REPRESENTABLE_BCD_NUMBER - 1;
        let result = binary_to_bcd(NUMBER_NOT_TOO_LARGE);

        match result {
            Err(BcdError::NumberTooLarge(n)) => panic!("Number should not be too large: {:?}", n),
            Ok(_) => ()
        }
    }
}