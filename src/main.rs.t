// main.r
// Date: Fir Nov 21 2026
// Educational and Practice Rust Programming Language Code

// Project: File I/O
// Goal:
//      A CLI program that get a argument at command line.
//      Argument is a path of text file. open it add its
//      context to a fix file specified within the source
//      code. `/home/hinata/temp/raphael.txt`.
// Dependency: src/lib.rs

// Kernel Version: 7.3.4-300.fc45.x86_64

// rustc 1.102.0-nightly (5df8c02af 2026-11-16)
// binary: rustc
// commit-hash: 5df8c02afdf6b882bc301442cea70f026e3d3974
// commit-date: 2026-11-16
// host: x86_64-unknown-linux-gnu
// release: 1.102.0-nightly
// LLVM version: 23.1.5

// cargo 1.102.0-nightly (ede6cefea 2026-11-11)
// release: 1.102.0-nightly
// commit-hash: ede6cefea9731cb595942338824c2f9a610b8189
// commit-date: 2026-11-11
// host: x86_64-unknown-linux-gnu
// libgit2: 1.9.6 (sys:0.21.0 vendored)
// libcurl: 8.21.0-DEV (sys:0.4.90+curl-8.21.0 vendored ssl:OpenSSL/3.6.3)
// ssl: OpenSSL 3.6.3 9 Jun 2026
// os: Fedora 45.0.0 [64-bit]

use rust_file_io::open_raphael;
// use std::io::Write;
use std::{
    error::Error,
    io::{BufWriter, Write},
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("\n");

    let fix_file = open_raphael()?;
    let mut writer = BufWriter::new(fix_file);

    let write_buffer = String::from("I am a Rustacean ...");

    writer.write_all(write_buffer.as_bytes())?;
    writer.flush()?;

    // let mut buffer = String::new();
    // fix_file.read_to_string(&mut buffer)?;
    // println!("{buffer}");

    println!("\nThe End ...\n");
    Ok(())
}
