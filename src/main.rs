use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Read, Write},
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

    /// Skip this many bytes
    #[arg(short, long, default_value_t = 0)]
    skip: usize,

    /// Write to this file
    #[arg(short, long)]
    output: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let mut reader =
        BufReader::new(File::open(cli.compressed).expect("Compressed source file should exist"));
    let mut buf = vec![0; cli.skip]; // a bit wasteful
    reader
        .read_exact(&mut buf)
        .expect("File should always contain at least skip bytes");
    let compressed_size = reader.read_le::<u32>();
    println!("{compressed_size}");
    let decompressed_size = reader.read_le::<u32>();
    println!("{decompressed_size}");
    let mut compressed_buf = vec![0; compressed_size as usize];
    reader
        .read_exact(&mut compressed_buf)
        .expect("Compressed source should always contain bytes indicated by compressed size");
    let mut decompressed_buf = vec![0; decompressed_size as usize];
    fastlz::decompress(&compressed_buf, &mut decompressed_buf)
        .expect("Decompressing should never fail on compressed data");
    BufWriter::new(File::create(cli.output).expect("Should be able to create output file"))
        .write_all(&decompressed_buf)
        .expect("Should be able to write decompressed content");
}
