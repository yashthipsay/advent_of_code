 use std::num::ParseIntError;

 enum OutOfRangeError {
 	TooLarge,
 	TooSmall,
 	NotEvenANumber,
 }

 impl From<ParseIntError> for OutOfRangeError {
 	fn from(_e: ParseIntError) -> Self {
 		Self::NotEvenANumber
 	}
 }

 fn string_to_int_in_range(s: String) -> Result<u32, OutOfRangeError> {
 	// Given: The u32::from_str_radix function returns Result<u32, ParseIntError>
 	let n: u32 = u32::from_str_radix(&s, 10)?;

 	match n {
 		n if n < 5 => Err(OutOfRangeError::TooSmall),
 		n if n > 100 => Err(OutOfRangeError::TooLarge),
 		n => Ok(n),
 	}
 }


 fn main(){}