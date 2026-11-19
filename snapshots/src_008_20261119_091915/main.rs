// main.rs
// Date: Thu Nov 19 2026
// Educational and Practice Rust Programming Language Code

// Project: File I/O
// Goal: Create a file and write into by BufWriter
// Dependency: Without any dependency

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

use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() -> Result<(), Box<dyn Error>> {
    println!("\n");

    let file_name = "/home/hinata/temp/raphael";

    // if the file exist it returns an Error.
    // Does not recreate it.
    let file = File::create_new(file_name)?;
    let mut br = BufWriter::new(file);

    let file_content = "I am learning Rust and Zig Programming Language ...".as_bytes();
    br.write_all(file_content)?;

    println!("{file_name} was created and the message has been written to it");

    println!("\nThe End ...\n");
    Ok(())
}
