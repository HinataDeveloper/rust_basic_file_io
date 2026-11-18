// lib.rs
// Date: Tue Nov 18 2026
// Educational and Practice Rust Programming Language Code

// Dependency: ...

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

use std::process::Command;
use std::error::Error;

/// # Errors
/// returns `Result<(), Box<dyn Error>>` if any error occur.
pub fn exec(cmd: &str, flags: &[&str]) -> Result<(), Box<dyn Error>> {
    let mut raw_output: String = String::new();

    let output_result = Command::new(cmd).args(flags).output();
    let output = match output_result {
        Ok(out) => out,
        Err(err) => return Err(Box::new(err)),
    };

    if output.status.success() {
        let raw_output_result = String::from_utf8(output.stdout);
        raw_output = match raw_output_result {
            Ok(out_str) => out_str,
            Err(err) => return Err(Box::new(err)),
        };
    }

    let line_output = raw_output.lines();
    for line in line_output {
        println!("{line}");
    }

    Ok(())
}
