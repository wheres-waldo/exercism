use Error::*;

const MASK: u8 = 0x7F;
const MORE: u8 = 0x80;
const PACKET_SIZE: u32 = 7;

#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|&value| {
            (0..=4)
                .rev()
                .map(|i| (value >> (i * PACKET_SIZE)) as u8 & MASK | if i == 0 { 0 } else { MORE })
                .skip_while(|&byte| byte == MORE)
                .collect::<Vec<u8>>()
        })
        .collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut numbers = Vec::new();

    if let Some(byte) = bytes.last() {
        if byte & MORE != 0 {
            return Err(IncompleteNumber);
        }
    }

    let overflows = bytes.windows(5).any(|window| {
        window[0..=3].iter().all(|&byte| byte & MORE != 0)
            && window[4] & MORE == 0
            && window[0] > 0x8F
    });

    if overflows {
        return Err(Overflow);
    }

    bytes.iter().fold(0, |acc, &byte| {
        let value = acc << PACKET_SIZE | (byte & MASK) as u32;

        if byte & MORE == 0 {
            numbers.push(value);
            0
        } else {
            value
        }
    });

    Ok(numbers)
}
