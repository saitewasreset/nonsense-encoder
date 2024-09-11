use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::str::FromStr;

use num_bigint::BigUint;

pub enum AppError {
    IOError(String),
    DecodeError(String),
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError(e) => write!(f, "IO error: {}", e),
            Self::DecodeError(e) => write!(f, "Decode error: {}", e),
        }
    }
}

impl Debug for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}

impl std::error::Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value.to_string())
    }
}

pub fn load_mapping(path: &Path) -> std::io::Result<Vec<String>> {
    let mut mapping_file = File::open(path)?;
    let mut mapping_file_content = String::new();

    mapping_file.read_to_string(&mut mapping_file_content)?;

    let mapping_list: Vec<String> = mapping_file_content
        .lines()
        .map(|s| String::from(s))
        .collect();

    Ok(mapping_list)
}

pub fn to_mapped(source: &[u8], mapping: &[String]) -> Vec<String> {
    let mut compressed_bytes_int = BigUint::from_bytes_le(&source);

    let mapping_len = mapping.len();

    let mut result: Vec<usize> = Vec::new();

    while &compressed_bytes_int > &BigUint::ZERO {
        let digit = &compressed_bytes_int % mapping_len;
        compressed_bytes_int /= mapping_len;

        result.push(digit.try_into().unwrap());
    }

    result
        .iter()
        .rev()
        .map(|&digit| mapping[digit].clone())
        .collect()
}

pub fn mapped_combine(mapped: &[String]) -> String {
    mapped.join("，")
}

pub fn mapped_split(mapped: &String) -> Vec<String> {
    mapped
        .split("，")
        .map(|s| String::from_str(s).unwrap())
        .collect()
}

pub fn to_source(mapped: &[String], mapping: &[String]) -> Result<Vec<u8>, AppError> {
    let mut mapped_to_idx = HashMap::with_capacity(mapping.len());

    mapping.iter().enumerate().for_each(|(idx, s)| {
        mapped_to_idx.insert(s, idx);
    });

    let mut bytes_int = BigUint::ZERO;

    for s in mapped {
        let idx = mapped_to_idx
            .get(s)
            .ok_or(AppError::DecodeError(format!("{} not in mapping", s)))?;

        bytes_int = bytes_int * mapping.len() + *idx;
    }

    Ok(bytes_int.to_bytes_le())
}

pub fn compress(source: &[u8]) -> Vec<u8> {
    let compressed_bytes: Vec<u8> = Vec::new();

    let mut zstd_encoder = zstd::Encoder::new(compressed_bytes, 15).unwrap();

    zstd_encoder.write_all(source).unwrap();

    let compressed_bytes = zstd_encoder.finish().unwrap();

    compressed_bytes
}

pub fn decompress(source: &[u8]) -> Vec<u8> {
    let mut decompressed_bytes: Vec<u8> = Vec::new();

    let mut zstd_decoder = zstd::Decoder::new(source).unwrap();

    zstd_decoder.read_to_end(&mut decompressed_bytes).unwrap();

    zstd_decoder.finish();

    decompressed_bytes
}
