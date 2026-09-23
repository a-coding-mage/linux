// SPDX-License-Identifier: GPL-2.0
//! Extract Alpha ELF/ECOFF load images or construct an SRM primary boot block.
// Copyright (C) 1996 David Mosberger-Tang.
mod bootblock;
use bootblock::{checksum, perror, BLOCK_SIZE};
use std::ffi::OsStr;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::os::unix::ffi::OsStrExt;

const BUFFER_SIZE: usize = 8192;
fn usage(errors: &mut Vec<u8>, program: &OsStr) -> i32 {
    errors.extend_from_slice(b"usage: ");
    errors.extend_from_slice(program.as_bytes());
    errors.extend_from_slice(b" [-v] -p file primary\n       ");
    errors.extend_from_slice(program.as_bytes());
    errors.extend_from_slice(b" [-vb] file [secondary]\n");
    1
}
fn message(errors: &mut Vec<u8>, program: &OsStr, input: Option<&OsStr>, text: &str) -> i32 {
    errors.extend_from_slice(program.as_bytes());
    errors.extend_from_slice(b": ");
    if let Some(input) = input {
        errors.extend_from_slice(input.as_bytes());
        errors.push(b' ');
    }
    errors.extend_from_slice(text.as_bytes());
    1
}
fn u16_at(data: &[u8], at: usize) -> u16 {
    u16::from_le_bytes(data[at..at + 2].try_into().unwrap())
}
fn u64_at(data: &[u8], at: usize) -> u64 {
    u64::from_le_bytes(data[at..at + 8].try_into().unwrap())
}
fn hex_address(value: u64) -> String {
    if value == 0 {
        format!("{value:016x}")
    } else {
        format!("{value:#016x}")
    }
}
fn read_chunk(input: &mut File, data: &mut [u8], errors: &mut Vec<u8>) -> bool {
    match input.read(data) {
        Ok(size) if size == data.len() => true,
        Ok(_) => {
            perror(errors, b"read", None);
            false
        }
        Err(err) => {
            perror(errors, b"read", Some(&err));
            false
        }
    }
}
fn write_chunk(output: &mut dyn Write, data: &[u8], label: &[u8], errors: &mut Vec<u8>) -> bool {
    if let Err(err) = output.write_all(data) {
        perror(errors, label, Some(&err));
        false
    } else {
        true
    }
}
fn run(errors: &mut Vec<u8>) -> i32 {
    let args: Vec<_> = std::env::args_os().collect();
    let program = &args[0];
    let (mut verbose, mut primary, mut pad) = (false, false, 0u64);
    let mut i = 1;
    while i < args.len() && args[i].as_bytes().starts_with(b"-") {
        for option in &args[i].as_bytes()[1..] {
            match option {
                b'v' => verbose = !verbose,
                b'b' => pad = BLOCK_SIZE as u64,
                b'p' => primary = true,
                _ => {}
            }
        }
        i += 1;
    }
    if i >= args.len() {
        return usage(errors, program);
    }
    let inname = &args[i];
    i += 1;
    let mut input = match File::open(inname) {
        Ok(file) => file,
        Err(err) => {
            perror(errors, b"open", Some(&err));
            return 1;
        }
    };
    let explicit_output = i < args.len();
    let mut output: Box<dyn Write> = if explicit_output {
        match OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&args[i])
        {
            Ok(file) => Box::new(file),
            Err(err) => {
                perror(errors, b"open", Some(&err));
                return 1;
            }
        }
    } else {
        Box::new(io::stdout())
    };
    if primary {
        if !explicit_output {
            return usage(errors, program);
        }
        let size = match input.metadata() {
            Ok(metadata) => metadata.len(),
            Err(err) => {
                perror(errors, b"fstat", Some(&err));
                return 1;
            }
        };
        let Some(size) = size
            .checked_add(BLOCK_SIZE as u64 - 1)
            .map(|v| v & !(BLOCK_SIZE as u64 - 1))
        else {
            return message(
                errors,
                program,
                Some(inname),
                "file size exceeds the boot-block format\n",
            );
        };
        let mut block = [0u8; BLOCK_SIZE];
        block[..20].copy_from_slice(b"Linux SRM bootblock\0");
        block[480..488].copy_from_slice(&(size / BLOCK_SIZE as u64).to_le_bytes());
        block[488..496].copy_from_slice(&1u64.to_le_bytes());
        checksum(&mut block);
        if !write_chunk(&mut output, &block, b"boot-block write", errors) {
            return 1;
        }
        if let Err(err) = output.flush() {
            perror(errors, b"boot-block write", Some(&err));
            return 1;
        }
        if !write_chunk(
            &mut io::stdout(),
            format!("{size}\n").as_bytes(),
            b"write",
            errors,
        ) {
            return 1;
        }
        return 0;
    }
    let mut buffer = [0u8; BUFFER_SIZE];
    let count = match input.read(&mut buffer) {
        Ok(count) => count,
        Err(err) => {
            perror(errors, b"read", Some(&err));
            return 1;
        }
    };
    let (mut offset, mut mem_size, mut file_size, address);
    if count >= 4 && buffer[..4] == *b"\x7fELF" {
        if count < 64 {
            return message(
                errors,
                program,
                Some(inname),
                "has a truncated ELF header\n",
            );
        }
        if u16_at(&buffer, 16) != 2 {
            return message(errors, program, Some(inname), "is not an ELF executable\n");
        }
        let machine = u16_at(&buffer, 18);
        if machine != 0x9026 {
            return message(
                errors,
                program,
                None,
                &format!("is not for this processor (e_machine={machine})\n"),
            );
        }
        if buffer[4] != 2 || buffer[5] != 1 {
            return message(
                errors,
                program,
                Some(inname),
                "is not a little-endian ELF64 file\n",
            );
        }
        let phnum = u16_at(&buffer, 56);
        if phnum != 1 {
            message(
                errors,
                program,
                None,
                &format!("{phnum} program headers (forgot to link with -N?)\n"),
            );
        }
        let entry = u64_at(&buffer, 24);
        let phoff = u64_at(&buffer, 32);
        if let Err(err) = input.seek(SeekFrom::Start(phoff)) {
            perror(errors, b"lseek", Some(&err));
            return 1;
        }
        if !read_chunk(&mut input, &mut buffer[..56], errors) {
            return 1;
        }
        offset = u64_at(&buffer, 8);
        let mut virtual_address = u64_at(&buffer, 16);
        file_size = u64_at(&buffer, 32);
        mem_size = u64_at(&buffer, 40);
        if virtual_address < entry {
            let delta = entry - virtual_address;
            let (Some(new_offset), Some(new_memory), Some(new_file)) = (
                offset.checked_add(delta),
                mem_size.checked_sub(delta),
                file_size.checked_sub(delta),
            ) else {
                return message(
                    errors,
                    program,
                    Some(inname),
                    "entry point lies outside the file-backed segment\n",
                );
            };
            offset = new_offset;
            mem_size = new_memory;
            file_size = new_file;
            virtual_address = entry;
        }
        address = virtual_address;
    } else {
        if count < 104 {
            return message(
                errors,
                program,
                Some(inname),
                "has a truncated ECOFF header\n",
            );
        }
        if u16_at(&buffer, 22) & 2 == 0 {
            return message(
                errors,
                program,
                Some(inname),
                "is not in executable format\n",
            );
        }
        if u16_at(&buffer, 20) != 80 {
            return message(
                errors,
                program,
                Some(inname),
                "has unexpected optional header size\n",
            );
        }
        if u64_at(&buffer, 24) & 0xffff != 0o407 {
            return message(errors, program, Some(inname), "is not an OMAGIC file\n");
        }
        offset = (104 + u16_at(&buffer, 2) as u64 * 64 + 15) & !15;
        let Some(size) = u64_at(&buffer, 32).checked_add(u64_at(&buffer, 40)) else {
            return message(errors, program, Some(inname), "ECOFF file size overflows\n");
        };
        file_size = size;
        let Some(size) = file_size.checked_add(u64_at(&buffer, 48)) else {
            return message(
                errors,
                program,
                Some(inname),
                "ECOFF memory size overflows\n",
            );
        };
        mem_size = size;
        address = u64_at(&buffer, 64);
    }
    if mem_size < file_size {
        return message(
            errors,
            program,
            Some(inname),
            "memory size is smaller than file size\n",
        );
    }
    if verbose {
        message(
            errors,
            program,
            None,
            &format!(
                "extracting {}-{} (at {offset:x})\n",
                hex_address(address),
                hex_address(address.wrapping_add(file_size))
            ),
        );
    }
    if let Err(err) = input.seek(SeekFrom::Start(offset)) {
        perror(errors, b"lseek", Some(&err));
        return 1;
    }
    if verbose {
        errors.extend_from_slice(program.as_bytes());
        errors.extend_from_slice(format!(": copying {file_size} byte from ").as_bytes());
        errors.extend_from_slice(inname.as_bytes());
        errors.push(b'\n');
    }
    let mut left = file_size;
    while left > 0 {
        let size = left.min(BUFFER_SIZE as u64) as usize;
        if !read_chunk(&mut input, &mut buffer[..size], errors) {
            return 1;
        }
        if !write_chunk(&mut output, &buffer[..size], b"write", errors) {
            return 1;
        }
        left -= size as u64;
    }
    if pad != 0 {
        let Some(size) = mem_size.checked_add(pad - 1).map(|size| size / pad * pad) else {
            return message(
                errors,
                program,
                Some(inname),
                "padded memory size overflows\n",
            );
        };
        mem_size = size;
    }
    left = mem_size - file_size;
    if left != 0 {
        message(
            errors,
            program,
            None,
            &format!("zero-filling bss and aligning to {pad} with {left} bytes\n"),
        );
        buffer.fill(0);
        while left > 0 {
            let size = left.min(BUFFER_SIZE as u64) as usize;
            if !write_chunk(&mut output, &buffer[..size], b"write", errors) {
                return 1;
            }
            left -= size as u64;
        }
    }
    if let Err(err) = output.flush() {
        perror(errors, b"write", Some(&err));
        return 1;
    }
    0
}
fn main() {
    let mut errors = Vec::new();
    let mut status = run(&mut errors);
    if io::stderr().lock().write_all(&errors).is_err() {
        status = 1;
    }
    std::process::exit(status)
}
