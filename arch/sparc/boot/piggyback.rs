// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) 1996,1997 Jakub Jelinek (jj@sunsite.mff.cuni.cz)
// Pete Zaitcev <zaitcev@yahoo.com> endian fixes for cross-compiles, 2000.
// Copyright (C) 2011 Sam Ravnborg <sam@ravnborg.org>
//! Append an initial ramdisk to a SPARC a.out tftpboot kernel in place.

#![forbid(unsafe_code)]

use std::env;
use std::ffi::{c_char, OsStr};
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Seek, SeekFrom, Write};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::MetadataExt;
use std::process::ExitCode;

type Result<T> = std::result::Result<T, Vec<u8>>;

fn error(name: &[u8], err: io::Error) -> Vec<u8> {
    let message = err.to_string();
    let message = message.split(" (os error ").next().unwrap_or(&message);
    [name, b": ", message.as_bytes(), b"\n"].concat()
}

fn align(value: u32, wide: bool) -> u32 {
    let mask = if wide { 8191 } else { 4095 };
    value.wrapping_add(mask) & !mask
}

// strtoul(base 16), including native unsigned-long overflow and truncation
// to the original unsigned-int start/end storage.
fn address(mut value: &[u8]) -> u32 {
    while matches!(value.first(), Some(b' ' | b'\t'..=b'\r')) {
        value = &value[1..];
    }
    let negative = value.first() == Some(&b'-');
    if matches!(value.first(), Some(b'+' | b'-')) {
        value = &value[1..];
    }
    if value.starts_with(b"0x") || value.starts_with(b"0X") {
        value = &value[2..];
    }
    let mut result = 0usize;
    let mut overflow = false;
    for byte in value {
        let digit = match byte {
            b'0'..=b'9' => byte - b'0',
            b'a'..=b'f' => byte - b'a' + 10,
            b'A'..=b'F' => byte - b'A' + 10,
            _ => break,
        };
        match result
            .checked_mul(16)
            .and_then(|n| n.checked_add(digit as usize))
        {
            Some(n) => result = n,
            None => {
                overflow = true;
                result = usize::MAX;
            }
        }
    }
    if overflow {
        usize::MAX as u32
    } else if negative {
        result.wrapping_neg() as u32
    } else {
        result as u32
    }
}

fn start_end(name: &OsStr) -> Result<(u32, u32)> {
    let file = File::open(name).map_err(|err| error(name.as_bytes(), err))?;
    let mut map = BufReader::new(file);
    let (mut start, mut end) = (0, 0);
    let mut buffer = [0; 1024];
    loop {
        // fgets reads at most 1023 bytes, including the newline when present.
        let mut line = Vec::new();
        match (&mut map).take(1023).read_until(b'\n', &mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => {}
        }
        // fgets only overwrites the new line and its terminating NUL. Keep
        // the suffix of the preceding line for the original fixed-column
        // comparisons, even when a following line is unusually short.
        buffer[..line.len()].copy_from_slice(&line);
        buffer[line.len()] = 0;
        let matches = |suffix: &[u8]| {
            [10, 18]
                .iter()
                .any(|offset| buffer[*offset..].split(|byte| *byte == 0).next() == Some(suffix))
        };
        if matches(b" _start\n") {
            start = address(&buffer);
        } else if matches(b" _end\n") {
            end = address(&buffer);
        }
    }
    if start == 0 || end == 0 {
        Err([
            b"Could not determine start and end from ",
            name.as_bytes(),
            b"\n",
        ]
        .concat())
    } else {
        Ok((start, end))
    }
}

fn seek(image: &mut File, offset: u64) -> Result<()> {
    image
        .seek(SeekFrom::Start(offset))
        .map(|_| ())
        .map_err(|err| error(b"lseek", err))
}

fn read_once(image: &mut File, buffer: &mut [u8], name: &[u8]) -> Result<()> {
    match image.read(buffer) {
        Ok(n) if n == buffer.len() => Ok(()),
        Ok(_) => Err(error(name, io::Error::from_raw_os_error(0))),
        Err(err) => Err(error(name, err)),
    }
}

fn write_once(image: &mut File, buffer: &[u8], name: &[u8]) -> Result<()> {
    match image.write(buffer) {
        Ok(n) if n == buffer.len() => Ok(()),
        Ok(_) => Err(error(name, io::Error::from_raw_os_error(0))),
        Err(err) => Err(error(name, err)),
    }
}

fn headers_offset(image: &mut File, name: &[u8]) -> Result<u64> {
    seek(image, 0)?;
    let mut buffer = [0; 1024];
    read_once(image, &mut buffer, name)?;
    if &buffer[40..44] == b"HdrS" {
        return Ok(40);
    }
    // Preserve ld2(char *): hosts with signed char sign-extend the low byte.
    let branch = (((buffer[34] as c_char as i32) << 8) | buffer[35] as c_char as i32) as u16;
    let offset = i64::from(branch) * 4 - 512 + 32;
    if offset < 0 {
        return Err(error(b"Calculated a negative offset, probably elftoaout generated an invalid image. Did you use a recent elftoaout ?", io::Error::from_raw_os_error(-22)));
    }
    seek(image, offset as u64)?;
    read_once(image, &mut buffer, name)?;
    for index in (0..512).step_by(4) {
        if &buffer[index..index + 4] == b"HdrS" {
            return Ok(offset as u64 + index as u64);
        }
    }
    Err([b"Couldn't find headers signature in ", name, b"\n"].concat())
}

fn run() -> Result<()> {
    let args: Vec<_> = env::args_os().collect();
    if args.len() != 5 {
        return Err(b"Usage: piggyback bits vmlinux.aout System.map fs_img.gz\n\tKernel image will be modified in place.\n".to_vec());
    }
    let wide = args[1] == "64";
    let ramdisk = fs::metadata(&args[4]).map_err(|err| error(args[4].as_bytes(), err))?;
    let size = ramdisk.len();
    let (start, end) = start_end(&args[3])?;
    let name = args[2].as_bytes();
    let mut image = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&args[2])
        .map_err(|err| error(name, err))?;
    let kernel = image.metadata().map_err(|err| error(name, err))?;
    if kernel.dev() == ramdisk.dev() && kernel.ino() == ramdisk.ino() {
        // Appending a file to itself through a second descriptor can keep
        // extending the file forever. Reject aliases before modifying it.
        return Err(b"Ramdisk image aliases kernel image.\n".to_vec());
    }
    let mut buffer = [0; 1024];
    read_once(&mut image, &mut buffer[..512], name)?;
    if buffer[..4] != [1, 3, 1, 7] {
        return Err(b"Not a.out. Don't blame me.\n".to_vec());
    }
    let offset = headers_offset(&mut image, name)?;
    seek(&mut image, offset + 10)?;
    let ramdisk_address = align(end.wrapping_add(32), wide);
    for (chunk, value) in
        buffer[..16]
            .chunks_exact_mut(4)
            .zip([0, 0x01000000, ramdisk_address, size as u32])
    {
        chunk.copy_from_slice(&value.to_be_bytes());
    }
    write_once(&mut image, &buffer[2..16], name)?;
    if wide {
        seek(&mut image, 4)?;
        let text_size = align(end.wrapping_add(32 + 8191), wide)
            .wrapping_sub(start & !0x3fffff)
            .wrapping_add(size as u32);
        buffer[..4].copy_from_slice(&text_size.to_be_bytes());
        buffer[4..12].fill(0);
        write_once(&mut image, &buffer[..12], name)?;
    }
    // C performs unsigned-int arithmetic before conversion to off_t, including
    // for kernels whose virtual start address is in the high half.
    seek(
        &mut image,
        u64::from(32u32.wrapping_sub(start).wrapping_add(ramdisk_address)),
    )?;
    let mut tail = File::open(&args[4]).map_err(|err| error(args[4].as_bytes(), err))?;
    loop {
        let size = tail
            .read(&mut buffer)
            .map_err(|err| error(args[4].as_bytes(), err))?;
        if size == 0 {
            break;
        }
        write_once(&mut image, &buffer[..size], name)?;
    }
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            let _ = io::stderr().write_all(&message);
            ExitCode::FAILURE
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
