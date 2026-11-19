// main.rs
// Date: Thu Nov 19 2026
// Educational and Practice Rust Programming Language Code

// Project: File I/O
// Goal: Check if a directory is exist. if not crate it.
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

use std::{fs::DirBuilder, path::Path};

fn main() {
    println!("\n");

    let my_path_one = Path::new("/home/hinata/temp");

    if my_path_one.exists() {
        println!("temp directory exist");
    } else {
        println!("{} directory not found ...", my_path_one.display());
        let db = DirBuilder::new();
        if matches!(db.create(my_path_one), Ok(())) {
            println!("{} was created successfully ...", my_path_one.display());
        } else {
            eprintln!(
                "--->>> error in create {} directory ...",
                my_path_one.display()
            );
        }
    }

    println!("\nThe End ...\n");
}
