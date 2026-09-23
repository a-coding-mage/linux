// SPDX-License-Identifier: GPL-2.0
//! Merge an Alpha disk label into a 512-byte SRM boot block.
mod bootblock;
use bootblock::{checksum, perror, BLOCK_SIZE};
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::unix::ffi::OsStrExt;

fn read_block(file: &mut File, name: &[u8], errors: &mut Vec<u8>) -> Option<[u8; BLOCK_SIZE]> {
    let mut block = [0; BLOCK_SIZE];
    let read = match file.read(&mut block) {
        Ok(count) if count == BLOCK_SIZE => return Some(block),
        Ok(count) => {
            perror(errors, name, None);
            count as isize
        }
        Err(err) => {
            perror(errors, name, Some(&err));
            -1
        }
    };
    errors.extend_from_slice(format!("expected {BLOCK_SIZE}, got {read}\n").as_bytes());
    None
}
fn run(errors: &mut Vec<u8>) {
    let args: Vec<_> = std::env::args_os().collect();
    if args.len() != 3 {
        errors.extend_from_slice(b"Usage: ");
        errors.extend_from_slice(args[0].as_bytes());
        errors.extend_from_slice(b" device lxboot\n");
        return;
    }
    let mut disk = match OpenOptions::new().read(true).write(true).open(&args[1]) {
        Ok(file) => file,
        Err(err) => {
            perror(errors, args[1].as_bytes(), Some(&err));
            return;
        }
    };
    let mut loader = match File::open(&args[2]) {
        Ok(file) => file,
        Err(err) => {
            perror(errors, args[2].as_bytes(), Some(&err));
            return;
        }
    };
    let Some(mut image) = read_block(&mut loader, b"lxboot read", errors) else {
        return;
    };
    let Some(previous) = read_block(&mut disk, b"bootblock read", errors) else {
        return;
    };
    // Alpha's disklabel is 148 bytes plus eight 16-byte partition entries.
    // Preserve it as wire bytes, including the filesystem's own checksum.
    image[64..340].copy_from_slice(&previous[64..340]);
    checksum(&mut image);
    if let Err(err) = disk.seek(SeekFrom::Start(0)) {
        perror(errors, b"bootblock seek", Some(&err));
        return;
    }
    if let Err(err) = disk.write_all(&image) {
        perror(errors, b"bootblock write", Some(&err));
    }
}
fn main() {
    let mut errors = Vec::new();
    run(&mut errors);
    let _ = std::io::stderr().lock().write_all(&errors);
    // Historical mkbb reports all failures through stderr but exits zero.
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
