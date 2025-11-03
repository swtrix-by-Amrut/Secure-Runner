use chrono::prelude::*;
use std::io::{self, Write};

use std::process::{Command, Stdio};

use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use hex_literal::hex;
use std::fs;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

fn decrypt_and_run() -> std::io::Result<()> {
    let key = hex!("--put-your-hex-encryption-key--");
    let content = fs::read("debug.bin")?;

    let iv = &content[..16];
    let ciphertext = &content[16..];

    let cipher = Aes256Cbc::new_from_slices(&key, iv).unwrap();
    let decrypted_data = cipher.decrypt_vec(ciphertext).unwrap();

    let mut child = Command::new("python")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start Python process");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin.write_all(&decrypted_data)?;
    }

    let output = child.wait_with_output()?;

    println!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
    println!("STDERR: {}", String::from_utf8_lossy(&output.stderr));

    Ok(())
}

fn main() {
    if let Err(e) = decrypt_and_run() {
        eprintln!("Error: {}", e);
    }
}
