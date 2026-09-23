// SPDX-License-Identifier: GPL-2.0-only
// Copyright 2015 Mentor Graphics Corporation.
//! Clear the ARM soft-float ELF flag so the vDSO supports both calling conventions.

use std::ffi::OsString;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::path::Path;

fn path_error(prefix: &str, path: &Path, error: io::Error) -> Vec<u8> {
    let mut message = prefix.as_bytes().to_vec();
    message.extend_from_slice(path.as_os_str().as_encoded_bytes());
    message.extend_from_slice(b": ");
    let error = error.to_string();
    message.extend_from_slice(
        error
            .split(" (os error ")
            .next()
            .unwrap_or(&error)
            .as_bytes(),
    );
    message.push(b'\n');
    message
}

fn hexadecimal(value: u32) -> String {
    // C's %#x prints zero without the 0x prefix.
    if value == 0 {
        "0".into()
    } else {
        format!("{value:#x}")
    }
}

fn run(input_path: &Path, output_path: &Path) -> Result<(), Vec<u8>> {
    let input =
        File::open(input_path).map_err(|error| path_error("Cannot open ", input_path, error))?;
    let metadata = input
        .metadata()
        .map_err(|error| path_error("Failed stat for ", input_path, error))?;
    if metadata.len() == 0 || metadata.is_dir() {
        // Preserve the diagnostics of mmap() for empty files and directories.
        let errno = if metadata.len() == 0 { 22 } else { 19 };
        return Err(path_error(
            "Failed to map ",
            input_path,
            io::Error::from_raw_os_error(errno),
        ));
    }
    let mut data = Vec::new();
    input
        .take(metadata.len())
        .read_to_end(&mut data)
        .map_err(|error| path_error("Failed to map ", input_path, error))?;
    if data.get(..4) != Some(b"\x7fELF") {
        return Err(b"Not an ELF file\n".to_vec());
    }
    if data.get(4) != Some(&1) {
        return Err(b"Unsupported ELF class\n".to_vec());
    }
    if data.len() < 52 {
        return Err(b"Truncated ELF header\n".to_vec());
    }
    let little = match data[5] {
        1 => true,
        2 => false,
        _ => return Err(b"Unsupported ELF data encoding\n".to_vec()),
    };
    let half = |offset| {
        let bytes = [data[offset], data[offset + 1]];
        if little {
            u16::from_le_bytes(bytes)
        } else {
            u16::from_be_bytes(bytes)
        }
    };
    if half(16) != 3 {
        return Err(b"Not a shared object\n".to_vec());
    }
    if half(18) != 40 {
        // The original diagnostic prints the host-order field, not its
        // decoded value, even for an opposite-endian image.
        let raw = u16::from_ne_bytes([data[18], data[19]]);
        return Err(
            format!("Unsupported architecture {}\n", hexadecimal(u32::from(raw))).into_bytes(),
        );
    }
    let flag_bytes = data[36..40].try_into().expect("four-byte ELF flags");
    let flags = if little {
        u32::from_le_bytes(flag_bytes)
    } else {
        u32::from_be_bytes(flag_bytes)
    };
    if flags & 0xff000000 != 0x05000000 {
        return Err(format!(
            "Unsupported EABI version {}\n",
            hexadecimal(flags & 0xff000000)
        )
        .into_bytes());
    }
    if flags & 0x400 != 0 {
        return Err(b"Unexpected hard-float flag set in e_flags\n".to_vec());
    }
    let mut output = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o600)
        .open(output_path)
        .map_err(|error| path_error("Cannot open ", output_path, error))?;
    output
        .set_len(metadata.len())
        .map_err(|error| path_error("Cannot truncate ", output_path, error))?;
    let output_metadata = output
        .metadata()
        .map_err(|error| path_error("Failed stat for ", output_path, error))?;
    if metadata.dev() == output_metadata.dev() && metadata.ino() == output_metadata.ino() {
        // Truncating the input inode also zeros the original MAP_PRIVATE
        // mapping. Retain this behavior for aliased input/output paths.
        data.fill(0);
    }
    if flags & 0x200 != 0 {
        let flags = flags & !0x200;
        data[36..40].copy_from_slice(&if little {
            flags.to_le_bytes()
        } else {
            flags.to_be_bytes()
        });
    }
    output
        .write_all(&data)
        .and_then(|()| output.sync_all())
        .map_err(|error| path_error("Failed to sync ", output_path, error))?;
    Ok(())
}

fn main() {
    let arguments: Vec<OsString> = std::env::args_os().collect();
    let result = if arguments.len() == 3 {
        run(Path::new(&arguments[1]), Path::new(&arguments[2]))
    } else {
        let mut message = b"Usage: ".to_vec();
        message.extend_from_slice(arguments[0].as_encoded_bytes());
        message.extend_from_slice(b" [infile] [outfile]\n");
        Err(message)
    };
    if let Err(message) = result {
        let mut stderr = io::stderr().lock();
        let _ = stderr.write_all(arguments[0].as_encoded_bytes());
        let _ = stderr.write_all(b": ");
        let _ = stderr.write_all(&message);
        if arguments.len() == 3 {
            // Kbuild must never retain an obsolete output after failure,
            // including validation errors before the output has been opened.
            let _ = fs::remove_file(&arguments[2]);
        }
        std::process::exit(1);
    }
}
