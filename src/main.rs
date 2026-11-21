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

use rust_file_io::get_string;
use rust_file_io::open_raphael;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    println!("\n");

    let mut write_buffer = String::from('\n');
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        write_buffer = get_string()?;
    } else if let Some(inp) = args.get(1) {
        write_buffer.clone_from(inp);
        write_buffer.push('\n');
    }

    let mut fix_file: File = open_raphael()?;

    fix_file.write_all(write_buffer.as_bytes())?;

    println!("Message was written ");

    println!("\nThe End ...\n");
    Ok(())
}
