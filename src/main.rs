use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::PathBuf,
};

use clap::Parser;
use ext::ByteReaderExt;
mod ext;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Input compressed file
    compressed: PathBuf,

    /// If this is disabled then don't output header info
    #[arg(long, default_value_t = true, action = clap::ArgAction::SetFalse)]
    header: bool,

    /// If this is enabled then compress instead
    #[arg(short, long, default_value_t = false)]
    compress: bool,

    /// Skip this many bytes
    #[arg(short, long, default_value_t = 0)]
    skip: usize,

    /// Write to this file
    #[arg(short, long)]
    output: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let file = File::open(cli.compressed).expect("Compressed source file should exist");
    let file_size = file.metadata().expect("File metadata must exist").len();
    let mut reader = BufReader::new(file);
    let mut skip_buf = vec![0; cli.skip]; // a bit wasteful
    reader
        .read_exact(&mut skip_buf)
        .expect("File should always contain at least skip bytes");

    let input_size = if cli.compress {
        file_size as u32
    } else {
        reader.read_le::<u32>()
    };

    let output_size = if cli.compress {
        std::cmp::max(66 + 8, input_size * 2)
    } else {
        reader.read_le::<u32>()
    };

    let mut input_buf = vec![0; input_size as usize];
    let mut output_buf = vec![0; output_size as usize];

    reader
        .read_exact(&mut input_buf)
        .expect("Input should always contain bytes indicated by input size");

    if cli.header {
        println!("input size: {input_size}, output size: {output_size}");
    }
    if cli.compress {
        let wrote_size = fastlz::compress(&input_buf, &mut output_buf[8..])
            .expect("Decompressing should never fail on compressed data")
            .len();
        output_buf[..4].copy_from_slice(&(wrote_size as u32).to_le_bytes());
        output_buf[4..8].copy_from_slice(&input_size.to_le_bytes());
    } else {
        fastlz::decompress(&input_buf, &mut output_buf)
            .expect("Decompressing should never fail on compressed data");
    }
    BufWriter::new(File::create(cli.output).expect("Should be able to create output file"))
        .write_all(&output_buf)
        .expect("Should be able to write decompressed content");
}
