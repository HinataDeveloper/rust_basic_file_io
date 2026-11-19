// main.rs
// Date: Thu Nov 19 2026
// Educational and Practice Rust Programming Language Code

// Project: File I/O
// Goal: ...
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

use std::{fs::File, io::Write};
// use std::io::BufReader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n");

    let file_name = "my_file_one";

    let file_result = File::create("my_file_one");
    let mut file = match file_result {
        Ok(f) => f,
        Err(err) => return Err(Box::new(err)),
    };

    println!("{file_name} was created ...");

    let content: &str = "This is my file one content";
    let write_size = file.write(content.as_bytes())?;

    println!("I wrote {write_size} size to the {file_name} file");

    println!("\nThe End ...\n");
    Ok(())
}
