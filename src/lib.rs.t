// lib.rs
// Date: Fri Nov 21 2026
// Educational and Practice Rust Programming Language Code

// Dependency: src/lib.rs has no dependency to any other crates.

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

use std::fs::File;
use std::io::Error;
use std::io::ErrorKind::{self};

const FIX_FILE: &str = "/home/hinata/temp/raphael.txt";
/// # Errors
/// return `Result<File, Error>` if any error ocurred
pub fn open_raphael() -> Result<File, Error> {
    let file_result = File::options().append(true).create_new(true).open(FIX_FILE);

    let file = match file_result {
        Ok(f) => f,
        Err(err) if err.kind() == ErrorKind::AlreadyExists => {
            let fix_file = File::open(FIX_FILE);
            fix_file?
            // match fix_file {
            //     Ok(ff) => ff,
            //     Err(err) => return Err(err),
            // }
        }
        Err(err) => return Err(err),
    };
    Ok(file)
}
