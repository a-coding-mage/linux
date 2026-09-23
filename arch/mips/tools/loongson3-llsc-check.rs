// SPDX-License-Identifier: GPL-2.0-only
//! Check Loongson 3 LL/SC sequences and branch-target synchronization.
#[path = "../../../scripts/elf-parse.rs"]
#[allow(dead_code)] // This checker needs section data, not symbol/name lookup.
mod elf;
#[path = "../boot/host_tool.rs"]
#[allow(dead_code)] // This CLI always prints a final status instead of finish().
mod host_tool;
use std::fs::File;
use std::io::{self, Read, Write};

fn is_ll(instruction: u32) -> bool {
    matches!(instruction >> 26, 0x30 | 0x34)
}
fn is_sc(instruction: u32) -> bool {
    matches!(instruction >> 26, 0x38 | 0x3c)
}
fn is_sync(instruction: u32) -> bool {
    instruction >> 11 == 0 && instruction & 0x3f == 0x0f
}
fn branch_offset(instruction: u32) -> Option<i32> {
    let branch = match instruction >> 26 {
        0x04..=0x07 | 0x14..=0x17 => true,
        1 => matches!((instruction>>16)&0x1f,0..=3|0x10..=0x13),
        _ => false,
    };
    branch.then_some((instruction as i16) as i32 + 1)
}
fn instruction(data: &[u8], at: u64) -> Option<u32> {
    let at = usize::try_from(at).ok()?;
    Some(u32::from_le_bytes(
        data.get(at..at.checked_add(4)?)?.try_into().ok()?,
    ))
}
fn problem(errors: &mut Vec<u8>, pc: u64, text: &str) -> bool {
    errors.extend_from_slice(format!("{pc:x}: {text}\n").as_bytes());
    false
}
fn check_ll(
    data: &[u8],
    code: &[u32],
    index: usize,
    file_offset: u64,
    address: u64,
    errors: &mut Vec<u8>,
) -> bool {
    let pc = address.wrapping_add(index as u64 * 4);
    if !is_sync(code[index - 1]) {
        return problem(errors, pc, "LL not preceded by sync");
    }
    let Some(sc) = code[index..].iter().position(|&insn| is_sc(insn)) else {
        return problem(errors, pc, "LL has no matching SC");
    };
    for (i, &insn) in code[index..index + sc].iter().enumerate() {
        let Some(offset) = branch_offset(insn) else {
            continue;
        };
        // Retain the original instruction filter (the upper comparison is
        // against SC's position, not the branch instruction's final target).
        if offset as i64 >= -(i as i64) && offset as i64 <= sc as i64 {
            continue;
        }
        let relative = (index as i128 + i as i128 + offset as i128) * 4;
        let target = u64::try_from(file_offset as i128 + relative)
            .ok()
            .and_then(|at| instruction(data, at));
        if !target.is_some_and(is_sync) {
            return problem(
                errors,
                pc.wrapping_add(i as u64 * 4),
                "Branch target not a sync",
            );
        }
    }
    true
}
fn check_code(data: &[u8], section: &elf::Section, bytes: &[u8], errors: &mut Vec<u8>) -> bool {
    let mut valid = true;
    if bytes.len() % 4 != 0 {
        valid = problem(errors, section.address, "Section size not a multiple of 4");
    }
    let code: Vec<u32> = bytes
        .chunks_exact(4)
        .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
        .collect();
    let Some(&first) = code.first() else {
        return valid;
    };
    if is_ll(first) {
        valid = problem(
            errors,
            section.address,
            "First instruction in section is an LL",
        );
    }
    for index in 1..code.len() {
        if is_ll(code[index]) {
            valid = check_ll(data, &code, index, section.offset, section.address, errors) && valid;
        }
    }
    valid
}
fn run(errors: &mut Vec<u8>) -> bool {
    let args: Vec<_> = std::env::args_os().collect();
    if args.len() < 2 {
        errors.extend_from_slice(b"Usage: loongson3-llsc-check /path/to/vmlinux\n");
        return false;
    }
    let mut file = match File::open(&args[1]) {
        Ok(file) => file,
        Err(err) => {
            errors.extend(host_tool::perror("Unable to open vmlinux", err));
            return false;
        }
    };
    let metadata = match file.metadata() {
        Ok(metadata) => metadata,
        Err(err) => {
            errors.extend(host_tool::perror("Unable to stat vmlinux", err));
            return false;
        }
    };
    if metadata.len() == 0 {
        errors.extend_from_slice(b"Unable to mmap vmlinux: Invalid argument\n");
        return false;
    }
    if !metadata.is_file() {
        errors.extend_from_slice(b"Unable to mmap vmlinux: No such device\n");
        return false;
    }
    let mut data = Vec::new();
    if let Err(err) = file.read_to_end(&mut data) {
        errors.extend(host_tool::perror("Unable to read vmlinux", err));
        return false;
    }
    if data.get(..4) != Some(b"\x7fELF") {
        errors.extend_from_slice(b"vmlinux is not an ELF?\n");
        return false;
    }
    if data.get(4) != Some(&2) {
        errors.extend_from_slice(b"vmlinux is not 64b?\n");
        return false;
    }
    if data.get(5) != Some(&1) {
        errors.extend_from_slice(b"vmlinux is not little endian?\n");
        return false;
    }
    let image = match elf::ElfFile::parse(&data, u32::MAX) {
        Ok(image) => image,
        Err(err) => {
            errors.extend_from_slice(format!("Invalid vmlinux: {err}\n").as_bytes());
            return false;
        }
    };
    let sections = match image.sections() {
        Ok(sections) => sections,
        Err(err) => {
            errors.extend_from_slice(format!("Invalid vmlinux: {err}\n").as_bytes());
            return false;
        }
    };
    for section in sections {
        if section.kind != 1 || section.flags & 4 == 0 {
            continue;
        }
        let bytes = match image.section_data(&section) {
            Ok(bytes) => bytes,
            Err(err) => {
                errors.extend_from_slice(format!("Invalid vmlinux: {err}\n").as_bytes());
                return false;
            }
        };
        if !check_code(&data, &section, bytes, errors) {
            return false;
        }
    }
    true
}
fn main() {
    let mut errors = Vec::new();
    let success = run(&mut errors);
    let mut status = if success { 0 } else { 1 };
    if io::stderr().lock().write_all(&errors).is_err() {
        status = 1;
    }
    let line = format!(
        "loongson3-llsc-check returns {}\n",
        if success { "success" } else { "failure" }
    );
    if let Err(err) = io::stdout().lock().write_all(line.as_bytes()) {
        host_tool::diagnostic(host_tool::perror("write", err));
        status = 1;
    }
    std::process::exit(status)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
