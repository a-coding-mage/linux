// SPDX-License-Identifier: GPL-2.0

use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::process;

const ELFCLASS32: u8 = 1;
const ELFCLASS64: u8 = 2;
const ELFDATA2LSB: u8 = 1;
const ELFDATA2MSB: u8 = 2;
const EI_CLASS: usize = 4;
const EI_DATA: usize = 5;
const SELFMAG: usize = 4;
const ELFMAG: [u8; SELFMAG] = [0x7f, b'E', b'L', b'F'];

#[repr(C)]
union Header {
    ehdr32: Elf32Ehdr,
    ehdr64: Elf64Ehdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Elf32Ehdr {
    e_ident: [u8; 16],
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u32,
    e_phoff: u32,
    e_shoff: u32,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Elf64Ehdr {
    e_ident: [u8; 16],
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

fn die(msg: &str) -> ! {
    eprint!("{msg}");
    process::exit(1);
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();
    let mut entry: u64;
    let mut hdr = [0u8; 64];

    if argc != 2 {
        die("Usage: elf-entry <elf-file>\n");
    }

    let mut file = match File::open(&argv[1]) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Unable to open input file: {error}");
            process::exit(1);
        }
    };

    let nread = match file.read(&mut hdr) {
        Ok(nread) => nread,
        Err(error) => {
            eprintln!("Unable to read input file: {error}");
            process::exit(1);
        }
    };
    if nread != hdr.len() {
        eprintln!("Unable to read input file");
        process::exit(1);
    }

    if hdr[..SELFMAG] != ELFMAG {
        die("Input is not an ELF\n");
    }

    match hdr[EI_CLASS] {
        ELFCLASS32 => {
            entry = match hdr[EI_DATA] {
                ELFDATA2LSB => u32::from_le_bytes(hdr[24..28].try_into().unwrap()) as u64,
                ELFDATA2MSB => u32::from_be_bytes(hdr[24..28].try_into().unwrap()) as u64,
                _ => die("Invalid ELF encoding\n"),
            };
            // Sign extend to form a canonical address
            entry = (entry as u32 as i32 as i64) as u64;
        }
        ELFCLASS64 => {
            entry = match hdr[EI_DATA] {
                ELFDATA2LSB => u64::from_le_bytes(hdr[24..32].try_into().unwrap()),
                ELFDATA2MSB => u64::from_be_bytes(hdr[24..32].try_into().unwrap()),
                _ => die("Invalid ELF encoding\n"),
            };
        }
        _ => die("Invalid ELF class\n"),
    }

    println!("0x{entry:016x}");
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
