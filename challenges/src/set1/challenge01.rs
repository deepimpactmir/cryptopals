use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::error::Error;
use std::fmt;

pub fn run() -> Result<(), Box<dyn Error>> {
    let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let base64 = hex_to_base64(hex)?;
    assert_eq!(
        base64,
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
    );
    println!("Success! Converted hex to base64:\n{}", base64);
    Ok(())
}

#[derive(Debug)]
pub enum ConversionError {
    InvalidHexString(String),
    InvalidLength,
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConversionError::InvalidHexString(s) => write!(f, "Invalid hex string: {}", s),
            ConversionError::InvalidLength => {
                write!(f, "Invalid length: hex string must have even length")
            }
        }
    }
}

impl Error for ConversionError {}

/// Converts a hexadecimal string to base64 encoding
///
/// # Arguments
/// * `hex_str` - A string slice that holds the hexadecimal string
///
/// # Returns
/// * `Result<String, ConversionError>` - The base64 encoded string or an error
///
/// # Examples
/// ```
/// let result = hex_to_base64("48656c6c6f20576f726c64").unwrap();
/// assert_eq!(result, "SGVsbG8gV29ybGQ=");
/// ```
pub fn hex_to_base64(hex_str: &str) -> Result<String, ConversionError> {
    // Check if the hex string length is even
    if hex_str.len() % 2 != 0 {
        return Err(ConversionError::InvalidLength);
    }

    // Convert hex string to bytes
    let bytes: Result<Vec<u8>, _> = (0..hex_str.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&hex_str[i..i + 2], 16)
                .map_err(|_| ConversionError::InvalidHexString(hex_str[i..i + 2].to_string()))
        })
        .collect();

    // Convert bytes to base64
    match bytes {
        Ok(bytes) => Ok(STANDARD.encode(bytes)),
        Err(e) => Err(e),
    }
}
