// This module is used at runtime to decode the binary format
// It's designed to be lightweight and suitable for WASM builds

use half::f16;
use std::io;

pub struct Sheet {
    pub ages: Vec<u8>,
    pub scores: Vec<f32>,
    pub age_results: Vec<Vec<f32>>,
}

pub struct Decoder<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Decoder<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    fn read_u8(&mut self) -> io::Result<u8> {
        if self.pos >= self.data.len() {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Unexpected end of data"));
        }
        let v = self.data[self.pos];
        self.pos += 1;
        Ok(v)
    }

    fn read_u16(&mut self) -> io::Result<u16> {
        if self.pos + 2 > self.data.len() {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Unexpected end of data"));
        }
        let v = u16::from_le_bytes([self.data[self.pos], self.data[self.pos + 1]]);
        self.pos += 2;
        Ok(v)
    }

    fn read_f16(&mut self) -> io::Result<f32> {
        let bits = self.read_u16()?;
        Ok(f16::from_bits(bits).to_f32())
    }

    fn read_f32(&mut self) -> io::Result<f32> {
        if self.pos + 4 > self.data.len() {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Unexpected end of data"));
        }
        let v = f32::from_le_bytes([
            self.data[self.pos],
            self.data[self.pos + 1],
            self.data[self.pos + 2],
            self.data[self.pos + 3],
        ]);
        self.pos += 4;
        Ok(v)
    }

    pub fn read_header(&mut self) -> io::Result<u16> {
        self.read_u16()
    }

    pub fn read_sheet(&mut self) -> io::Result<Sheet> {
        let age_count = self.read_u8()? as usize;
        let score_count = self.read_u8()? as usize;

        let mut ages = Vec::with_capacity(age_count);
        for _ in 0..age_count {
            ages.push(self.read_u8()?);
        }

        let mut scores = Vec::with_capacity(score_count);
        for _ in 0..score_count {
            let encoded = self.read_u8()?;
            scores.push(encoded as f32 / 2.0);
        }

        let mut age_results = Vec::with_capacity(age_count);
        for _ in 0..age_count {
            if score_count == 0 {
                age_results.push(Vec::new());
                continue;
            }

            let mut column = Vec::with_capacity(score_count);
            let base = self.read_f32()?;
            column.push(base);

            let mut prev = base;
            for _ in 1..score_count {
                let delta = self.read_f16()?;
                let value = prev + delta;
                column.push(value);
                prev = value;
            }
            age_results.push(column);
        }

        Ok(Sheet {
            ages,
            scores,
            age_results,
        })
    }
}

pub fn decode_all(data: &[u8]) -> io::Result<Vec<Sheet>> {
    let mut decoder = Decoder::new(data);
    let sheet_count = decoder.read_header()? as usize;

    let mut sheets = Vec::with_capacity(sheet_count);
    for _ in 0..sheet_count {
        sheets.push(decoder.read_sheet()?);
    }

    Ok(sheets)
}
