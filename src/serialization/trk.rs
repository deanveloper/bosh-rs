//! Module for deserializing LRA tracks, aka .trk files.

use std::collections::HashSet;
use std::io::Read;
use std::slice;

use anyhow::{anyhow, Context, Error};
use read_from::{LittleEndian, ReadFrom};

use crate::Vector2D;

pub const FEATURE_SONGINFO: &str = "SONGINFO";

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum TrkLineType {
    Scenery = 0,
    Blue = 1,
    Red = 2,
}

#[derive(Copy, Clone)]
pub struct TrkLineFlags(pub u8);

#[derive(Clone)]
pub struct TrkHeader {
    pub version: u8,
    pub features: HashSet<String>,

    pub song: Option<String>,
    pub start_position: Vector2D,
}

impl TrkLineFlags {
    pub fn flipped(&self) -> bool {
        self.0 & 0b10000000 > 0
    }
    pub fn extensions(&self) -> (bool, bool) {
        (self.0 & 0b01000000 > 0, self.0 & 0b00100000 > 0)
    }
    pub fn line_type(&self) -> TrkLineType {
        (self.0 & 0b00011111).into()
    }
}

impl ReadFrom for TrkLineFlags {
    type Error = Error;

    fn read_from<R: Read>(mut input: R) -> Result<Self, Self::Error> {
        let mut flags = 0;
        input
            .read_exact(slice::from_mut(&mut flags))
            .context("error while reading line flags")?;

        Ok(TrkLineFlags(flags))
    }
}

impl From<u8> for TrkLineType {
    fn from(value: u8) -> TrkLineType {
        match value {
            0 => TrkLineType::Scenery,
            1 => TrkLineType::Blue,
            2 => TrkLineType::Red,
            _ => TrkLineType::Scenery,
        }
    }
}

impl ReadFrom for TrkHeader {
    type Error = Error;

    fn read_from<R: Read>(mut input: R) -> Result<Self, Self::Error> {
        let magic =
            <[u8; 4]>::read_from(&mut input).context("error while reading header, magic value")?;
        if magic != [b'T', b'R', b'K', 0xF2] {
            return Err(anyhow!("magic value was not correct"));
        }

        let version = u8::read_from(&mut input).context("error while reading header, version")?;

        let features_length: u16 = LittleEndian::read_from(&mut input)
            .context("error while reading header, length of features-string")?
            .0;

        let mut features_string = vec![0; features_length as usize];
        input
            .read_exact(features_string.as_mut_slice())
            .with_context(|| {
                format!(
                    "error while reading header, in features-string of length {features_length}"
                )
            })?;

        let features: HashSet<String> = String::from_utf8_lossy(features_string.as_slice())
            .split(';')
            .map(|s| s.to_owned())
            .collect();

        let song = if features.contains(FEATURE_SONGINFO) {
            let song_length =
                u8::read_from(&mut input).context("error while reading header, song length")?;
            let mut song = vec![0; song_length as usize];

            input.read_exact(song.as_mut_slice()).with_context(|| {
                format!("error while reading header, in song of length {song_length}")
            })?;

            let song = String::from_utf8_lossy(features_string.as_slice())
                .split(';')
                .map(|s| s.to_owned())
                .collect();

            Some(song)
        } else {
            None
        };

        let start_position = Vector2D::read_from(&mut input)
            .context("error while reading header, start position")?;

        Ok(TrkHeader {
            version,
            features,
            song,
            start_position,
        })
    }
}

// TODO - Line, Track
