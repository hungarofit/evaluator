// This module is ONLY used at build-time (in build.rs)
// It should NOT be included in the WASM build

use half::f16;
use std::io::{self, Write};

pub struct Sheet {
    pub ages: Vec<u8>,
    pub scores: Vec<f32>,
    pub age_results: Vec<Vec<f32>>,
}

pub struct Encoder<W: Write> {
    writer: W,
}

impl<W: Write> Encoder<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    fn write_u8(&mut self, v: u8) -> io::Result<()> {
        self.writer.write_all(&[v])
    }

    fn write_u16(&mut self, v: u16) -> io::Result<()> {
        self.writer.write_all(&v.to_le_bytes())
    }

    fn write_f16(&mut self, v: f32) -> io::Result<()> {
        self.writer.write_all(&f16::from_f32(v).to_le_bytes())
    }

    fn write_f32(&mut self, v: f32) -> io::Result<()> {
        self.writer.write_all(&v.to_le_bytes())
    }

    pub fn write_header(&mut self, sheet_count: u16) -> io::Result<()> {
        self.write_u16(sheet_count)
    }

    pub fn write_sheet(&mut self, sheet: &Sheet) -> io::Result<()> {
        if sheet.ages.is_empty() || sheet.scores.is_empty() || sheet.age_results.len() != sheet.ages.len() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid sheet dimensions"));
        }

        self.write_u8(sheet.ages.len() as u8)?;
        self.write_u8(sheet.scores.len() as u8)?;

        for &age in &sheet.ages {
            self.write_u8(age)?;
        }

        for &score in &sheet.scores {
            self.write_u8((score * 2.0).round() as u8)?;
        }

        for age_results in &sheet.age_results {
            if !age_results.is_empty() {
                self.write_f32(age_results[0])?;
                for i in 1..age_results.len() {
                    self.write_f16(age_results[i] - age_results[i - 1])?;
                }
            }
        }

        Ok(())
    }

    pub fn into_inner(self) -> W {
        self.writer
    }
}

pub fn encode_all<W: Write>(sheets: &[Sheet], writer: W) -> io::Result<W> {
    let mut encoder = Encoder::new(writer);
    encoder.write_header(sheets.len() as u16)?;
    for sheet in sheets {
        encoder.write_sheet(sheet)?;
    }
    Ok(encoder.into_inner())
}
