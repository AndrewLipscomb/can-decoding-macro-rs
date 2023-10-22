#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use core::default::Default;
use proc_macro_derive_crate::CanDecode;

#[derive(CanDecode)]
struct Ding {
    #[can_extract(offset = 0)]
    a: u32,
    #[can_extract(offset = 4)]
    b: u32,
}

#[derive(Default)]
struct Dong {
    a: u32,
    b: u32,
}

fn advance_token<'a, T: Sized>(
    offset: &'a mut usize,
    frame: &'a [u8],
) -> Result<&'a [u8], can_extract::Error> {
    let val = std::mem::size_of::<T>();
    let next = *offset + val;
    let slice = frame
        .get(*offset..next)
        .ok_or_else(|| can_extract::Error::InvalidSlicingLength)?;
    *offset = next;
    Ok(slice)
}

impl can_extract::CanDecode for Dong {
    fn from_socketcan(frame: [u8; 8]) -> Result<Self, can_extract::Error> {
        // let (int_bytes, rest) = frame.split_at(std::mem::size_of::<u32>());
        let mut offset: usize = 0;
        let val = Dong {
            a: u32::from_ne_bytes(
                advance_token::<u32>(&mut offset, &frame)?
                    .try_into()
                    .map_err(|_| can_extract::Error::InvalidBytesConversion)?,
            ),
            b: u32::from_ne_bytes(
                advance_token::<u32>(&mut offset, &frame)?
                    .try_into()
                    .map_err(|_| can_extract::Error::InvalidBytesConversion)?,
            ),
        };
        Ok(val)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run_things() {
        let data: [u8; 8] = [0, 0, 0, 0, 1, 0, 0, 0];
        let ding: Ding =
            can_extract::CanDecode::from_socketcan(data).expect("Did not decode correctly");
        assert!(ding.a == 0);
        assert!(ding.b == 1);
        // answer();

        let dong: Dong =
            can_extract::CanDecode::from_socketcan(data).expect("Did not get error yay");
    }
}
