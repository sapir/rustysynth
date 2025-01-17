#![allow(dead_code)]

use std::error::Error;
use std::io;
use std::io::Read;
use std::slice;
use std::str;

#[allow(unused)]
#[non_exhaustive]
pub(crate) struct BinaryReader {}

impl BinaryReader {
    pub(crate) fn read_i8<R: Read>(reader: &mut R) -> Result<i8, io::Error> {
        let mut data: [u8; 1] = [0; 1];
        reader.read_exact(&mut data)?;
        Ok(i8::from_le_bytes(data))
    }

    pub(crate) fn read_u8<R: Read>(reader: &mut R) -> Result<u8, io::Error> {
        let mut data: [u8; 1] = [0; 1];
        reader.read_exact(&mut data)?;
        Ok(u8::from_le_bytes(data))
    }

    pub(crate) fn read_i16<R: Read>(reader: &mut R) -> Result<i16, io::Error> {
        let mut data: [u8; 2] = [0; 2];
        reader.read_exact(&mut data)?;
        Ok(i16::from_le_bytes(data))
    }

    pub(crate) fn read_u16<R: Read>(reader: &mut R) -> Result<u16, io::Error> {
        let mut data: [u8; 2] = [0; 2];
        reader.read_exact(&mut data)?;
        Ok(u16::from_le_bytes(data))
    }

    pub(crate) fn read_i32<R: Read>(reader: &mut R) -> Result<i32, io::Error> {
        let mut data: [u8; 4] = [0; 4];
        reader.read_exact(&mut data)?;
        Ok(i32::from_le_bytes(data))
    }

    pub(crate) fn read_i16_big_endian<R: Read>(reader: &mut R) -> Result<i16, io::Error> {
        let mut data: [u8; 2] = [0; 2];
        reader.read_exact(&mut data)?;
        Ok(i16::from_be_bytes(data))
    }

    pub(crate) fn read_i32_big_endian<R: Read>(reader: &mut R) -> Result<i32, io::Error> {
        let mut data: [u8; 4] = [0; 4];
        reader.read_exact(&mut data)?;
        Ok(i32::from_be_bytes(data))
    }

    pub(crate) fn read_i32_variable_length<R: Read>(reader: &mut R) -> Result<i32, Box<dyn Error>> {
        let mut acc: i32 = 0;
        let mut count: i32 = 0;

        loop {
            let value = BinaryReader::read_u8(reader)? as i32;
            acc = (acc << 7) | (value & 127);
            if (value & 128) == 0 {
                break;
            }
            count += 1;
            if count == 4 {
                return Err(
                    format!("The length of the value must be equal to or less than 4.").into(),
                );
            }
        }

        Ok(acc)
    }

    pub(crate) fn read_four_cc<R: Read>(reader: &mut R) -> Result<String, Box<dyn Error>> {
        let mut data: [u8; 4] = [0; 4];
        reader.read_exact(&mut data)?;

        for i in 0..4 {
            let value = data[i];
            if !(32 <= value && value <= 126) {
                data[i] = 63; // '?'
            }
        }

        Ok(str::from_utf8(&data)?.to_string())
    }

    pub(crate) fn read_fixed_length_string<R: Read>(
        reader: &mut R,
        length: i32,
    ) -> Result<String, Box<dyn Error>> {
        let mut data: Vec<u8> = vec![0; length as usize];
        reader.read_exact(&mut data)?;

        let mut actual_length: i32 = 0;
        for i in 0..length {
            if data[i as usize] == 0 {
                break;
            }
            actual_length += 1;
        }

        Ok(str::from_utf8(&data[0..actual_length as usize])?.to_string())
    }

    pub(crate) fn discard_data<R: Read>(reader: &mut R, size: i32) -> Result<(), io::Error> {
        let mut data: Vec<u8> = vec![0; size as usize];
        reader.read_exact(&mut data)
    }

    pub(crate) fn read_wave_data<R: Read>(
        reader: &mut R,
        size: i32,
    ) -> Result<Vec<i16>, io::Error> {
        let length = size as usize / 2;
        let mut samples: Vec<i16> = vec![0; length];

        let ptr = samples.as_mut_ptr() as *mut u8;
        let data = unsafe { slice::from_raw_parts_mut(ptr, size as usize) };
        reader.read_exact(data)?;

        Ok(samples)
    }
}
