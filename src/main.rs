use clap::Parser;
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;

mod encoder;

use encoder::{
    compress, decompress, load_mapping, mapped_combine, mapped_split, to_mapped, to_source,
    AppError,
};
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Codec file to use
    codec: PathBuf,

    /// input file, defaults to stdin
    input: Option<PathBuf>,
    /// output file, defaults to stdout
    output: Option<PathBuf>,

    /// encode mode, defaults to decode
    #[arg(short, long)]
    encode: bool,
}

fn encode(
    mapping_path: &PathBuf,
    input_source: &[u8],
    output_path: Option<&PathBuf>,
) -> Result<(), AppError> {
    let mapping_list: Vec<String> = load_mapping(mapping_path)?;

    let mapping_len = mapping_list.len();
    println!("codec len: {}", mapping_len);

    let source_bytes_len = input_source.len();
    println!("source bytes len: {} bytes", source_bytes_len);

    let compressed_bytes = compress(input_source);

    println!(
        "compressed bytes len: {} bytes ({:.2}%)",
        compressed_bytes.len(),
        (compressed_bytes.len() as f64 / source_bytes_len as f64) * 100.0
    );

    let encoded_list = to_mapped(&compressed_bytes, &mapping_list);

    let encoded_str = mapped_combine(&encoded_list);

    println!("encoded len: {}", encoded_str.chars().count());

    println!(
        "encoded bytes len: {} bytes ({:.2}%)",
        encoded_str.len(),
        (encoded_str.len() as f64 / source_bytes_len as f64) * 100.0
    );

    if let Some(output_file_path) = output_path {
        fs::File::create(output_file_path)?.write_all(encoded_str.as_bytes())?;
    } else {
        println!("{}", encoded_str);
    }

    Ok(())
}

fn decode(
    mapping_path: &PathBuf,
    input_source: &[u8],
    output_path: Option<&PathBuf>,
) -> Result<(), AppError> {
    let mapping_list: Vec<String> = load_mapping(mapping_path)?;

    let mapping_len = mapping_list.len();
    println!("codec len: {}", mapping_len);

    let source_bytes_len = input_source.len();
    println!("source bytes len: {} bytes", source_bytes_len);

    let input_string = String::from_utf8(Vec::from(input_source))
        .map_err(|e| AppError::DecodeError(format!("cannot decode input as utf-8: {}", e)))?;

    let input_list = mapped_split(&input_string);

    let decoded = to_source(&input_list, &mapping_list)?;
    println!("decoded bytes len: {} bytes", decoded.len());

    let decompressed_bytes = decompress(&decoded);

    println!(
        "decompressed bytes len: {} bytes ({:.2}%)",
        decompressed_bytes.len(),
        (decompressed_bytes.len() as f64 / source_bytes_len as f64) * 100.0
    );

    if let Some(output_file_path) = output_path {
        fs::File::create(output_file_path)?.write_all(&decompressed_bytes)?;
    } else {
        let decompressed_string_result = String::from_utf8(decompressed_bytes);

        if let Ok(decompressed_string) = decompressed_string_result {
            println!("{}", decompressed_string);
        } else {
            println!("Error while decoding decompressed bytes as utf-8 string, binary data or wrong mapping?");
        }
    }

    Ok(())
}

fn main() -> Result<(), AppError> {
    let cli = Cli::parse();

    let mapping_path = &cli.codec;

    let mut source = Vec::new();

    if let Some(input_file_path) = cli.input {
        fs::File::open(input_file_path)?.read_to_end(&mut source)?;
    } else {
        io::stdin().read_to_end(&mut source)?;
    }

    if cli.encode {
        encode(&mapping_path, &source, cli.output.as_ref())?;
    } else {
        let source_str = String::from_utf8(source)
            .map_err(|e| AppError::DecodeError(format!("cannot decode input as utf-8: {}", e)))?;
        let source_str = source_str.trim();
        decode(&mapping_path, source_str.as_bytes(), cli.output.as_ref())?;
    }

    Ok(())
}
