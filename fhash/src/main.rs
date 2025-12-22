use sha2::{Sha256, Digest};
use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::process::exit;

fn hash_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];

    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage: sha256 <file> [file ...]");
        exit(1);
    }

    for path in args {
        match hash_file(&path) {
            Ok(hash) => println!("{}  {}", hash, path),
            Err(e) => eprintln!("{}: {}", path, e),
        }
    }
}
