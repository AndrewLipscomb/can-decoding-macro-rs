#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use core::default::Default;
use proc_macro_derive_crate::CanDecode;

fn divide_by_1000(bytes: &[u8]) -> Result<f32, can_extract::Error> {
    // Assert we got a 4 byte chunk
    assert!(bytes.len() == std::mem::size_of::<u16>());

    // Extract a BE 16 byte value
    let original = u16::from_be_bytes(
        bytes
            .try_into()
            .map_err(|e| can_extract::Error::InvalidSlicingLength)?,
    );

    // And convert to float, with division
    Ok((original as f32) / 1000.0)
}

/// A simple test struct
#[derive(CanDecode, Debug)]
struct TestStruct {
    // All items must have an offset, indicating where they start reading in the buffer
    // The reader will extract std::mem::size_of<T> for a struct member, unless told to use an extract = u8 val
    // These can be passed to a decoder using use_decoder, else specify use_big_endian for BE, or leave blank for LE

    // Start at byte 0, extract 2 bytes and pass to u16::from_le_bytes()
    #[can_extract(offset = 0)]
    a: u16,
    // Start at byte 2, extract 2 bytes, and pass to u16::from_be_bytes()
    #[can_extract(offset = 2, use_big_endian)]
    b: u16,
    // Start at byte 6, skipping bytes 4,5 extract 2 bytes for a u16 base value, and convert to f32 via the divide_by_1000 func
    #[can_extract(offset = 6, extract = 2, use_decoder = "divide_by_1000")]
    c: f32,
}

#[cfg(test)]
mod test {
    use super::*;

    use float_cmp::ApproxEq;

    #[test]
    fn run_things() {
        let data: [u8; 8] = [5, 0, 0, 5, 0, 0, 0, 1];
        let val: TestStruct =
            can_extract::CanDecode::from_socketcan(data).expect("Did not decode correctly");
        dbg!(&val);
        assert!(val.a == 5);
        assert!(val.b == 5);
        assert!(val.c.approx_eq(0.001, (0.0, 2)));
    }
}
