# fhash
A simple command-line tool to compute SHA-256 hashes of files.

## Installation
```bash 
git clone https://github.com/hupeyaszih/fhash.git
cd fhash
cargo build --release
```
The compiled binary will be available at:
```text
target/release/fhash
```

You can run it directly:
```text
./target/release/fhash <file>
```
## Usage
### Hash a file
```bash
./target/release/fhash ./testFile.txt
```
```text
Output: a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e ./testFile.txt
```
### Hash multiple files
```bash
./target/release/fhash ./testFile.txt ./testFile2.txt 
```
Output:
```text
a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e  ./testFile.txt
f3675e49cf596ee99dd0a6acca882cdd222adb80a8d0dce58a2d36e9466f7a4f  ./testFile2.txt
```
## Notes
This tool computes SHA-256 hashes by reading files in fixed-size chunks, so it can handle very large files without high memory usage.
Hashes are calculated from the file contents, not from filenames or metadata.
A small change in a file will result in a completely different hash value.
If multiple files are provided, each file is processed independently.
This tool is intended for file integrity verification and general hashing purposes, not for password storage.
Output is deterministic: the same file will always produce the same hash.