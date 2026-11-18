// main.rs
// Date: Tue Nov 18 2026
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

use rust_file_io::{DEST_FILE, SRC_FILE};

fn main() {
    println!("\n");

    // Copy file operation
    let result = std::fs::copy(SRC_FILE, DEST_FILE);
    match result {
        Ok(size) => println!("rust_version.txt was copied to rv.txt file with {size} size."),
        Err(err) => {
            eprintln!("--->>> {err}");
        }
    }

    // Check if rv.txt file is exist.
    let exist_result = std::fs::exists(DEST_FILE);
    let exist = match exist_result {
        Ok(b) => b,
        Err(err) => {
            println!("--->>> {err}");
            false
        }
    };

    // if rv.txt file exist remove it.
    if exist {
        let remove_result = std::fs::remove_file("rv.txt");
        match remove_result {
            Ok(()) => println!("rv.txt file was removed ..."),
            Err(err) => println!("--->>> {err}"),
        }
    } else {
        println!("{DEST_FILE} file not found ...");
    }

    println!("\nThe End ...\n");
}
