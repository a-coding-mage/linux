// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2023 - Google LLC
 * Author: Ard Biesheuvel <ardb@google.com>
 */

use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

// Equivalent declarations for the ELF types and constants supplied by <elf.h>.
#[repr(C)]
struct Elf64_Ehdr {
    e_ident: [u8; 16], e_type: u16, e_machine: u16, e_version: u32,
    e_entry: u64, e_phoff: u64, e_shoff: u64, e_flags: u32,
    e_ehsize: u16, e_phentsize: u16, e_phnum: u16, e_shentsize: u16,
    e_shnum: u16, e_shstrndx: u16,
}
#[repr(C)]
struct Elf64_Shdr {
    sh_name: u32, sh_type: u32, sh_flags: u64, sh_addr: u64,
    sh_offset: u64, sh_size: u64, sh_link: u32, sh_info: u32,
    sh_addralign: u64, sh_entsize: u64,
}
#[repr(C)]
struct Elf64_Rela { r_offset: u64, r_info: u64, r_addend: i64 }

const HOST_ORDER: u8 = if cfg!(target_endian = "little") { 1 } else { 2 };
const EI_DATA: usize = 5;
const SHT_RELA: u32 = 4;
const SHF_ALLOC: u64 = 2;
const SHF_EXECINSTR: u64 = 4;
const R_AARCH64_ABS64: u32 = 257;
const R_AARCH64_PREL64: u32 = 275;

extern "C" {
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn fstat(fd: c_int, buf: *mut libc::stat) -> c_int;
    fn mmap(addr: *mut c_void, len: usize, prot: c_int, flags: c_int, fd: c_int, off: libc::off_t) -> *mut c_void;
    fn munmap(addr: *mut c_void, len: usize) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn unlink(path: *const c_char) -> c_int;
    fn fprintf(stream: *mut libc::FILE, format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    static mut stderr: *mut libc::FILE;
}

static mut ehdr: *mut Elf64_Ehdr = ptr::null_mut();
static mut shdr: *mut Elf64_Shdr = ptr::null_mut();
static mut strtab: *const c_char = ptr::null();
static mut swap: bool = false;

unsafe fn swab_elfxword(val: u64) -> u64 { if swap { val.swap_bytes() } else { val } }
unsafe fn swab_elfword(val: u32) -> u32 { if swap { val.swap_bytes() } else { val } }
unsafe fn swab_elfhword(val: u16) -> u16 { if swap { val.swap_bytes() } else { val } }

unsafe fn elf64_r_type(info: u64) -> u32 { (info & 0xFFFF_FFFF) as u32 }

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let stat_buf: libc::stat = std::mem::zeroed();
    let mut stat_buf = stat_buf;
    let mut fd: c_int;
    let mut ret: c_int;

    if argc < 3 {
        fprintf(stderr, b"file arguments missing\0".as_ptr() as *const c_char);
        exit(libc::EXIT_FAILURE);
    }

    fd = open(*argv.add(1), libc::O_RDWR);
    if fd < 0 {
        fprintf(stderr, b"failed to open %s\n\0".as_ptr() as *const c_char, *argv.add(1));
        exit(libc::EXIT_FAILURE);
    }
    ret = fstat(fd, &mut stat_buf);
    if ret < 0 {
        fprintf(stderr, b"failed to stat() %s\n\0".as_ptr() as *const c_char, *argv.add(1));
        exit(libc::EXIT_FAILURE);
    }
    ehdr = mmap(ptr::null_mut(), stat_buf.st_size as usize, libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED, fd, 0) as *mut Elf64_Ehdr;
    if ehdr == libc::MAP_FAILED as *mut Elf64_Ehdr {
        fprintf(stderr, b"failed to mmap() %s\n\0".as_ptr() as *const c_char, *argv.add(1));
        exit(libc::EXIT_FAILURE);
    }

    swap = (*ehdr).e_ident[EI_DATA] != HOST_ORDER;
    shdr = (ehdr as *mut u8).add(swab_elfxword((*ehdr).e_shoff) as usize) as *mut Elf64_Shdr;
    let strtab_ptr = shdr.add(swab_elfhword((*ehdr).e_shstrndx) as usize);
    strtab = (ehdr as *mut u8).add(swab_elfxword((*strtab_ptr).sh_offset) as usize) as *const c_char;

    for i in 0..swab_elfhword((*ehdr).e_shnum) as usize {
        let mut prel64 = false;
        if swab_elfword((*shdr.add(i)).sh_type) != SHT_RELA { continue; }
        let info = swab_elfword((*shdr.add(i)).sh_info) as usize;
        let flags = swab_elfxword((*shdr.add(info)).sh_flags);
        if (flags & (SHF_ALLOC | SHF_EXECINSTR)) != SHF_ALLOC { continue; }
        let name = strtab.add(swab_elfword((*shdr.add(info)).sh_name) as usize);
        if CStr::from_ptr(name).to_bytes().windows(b".rodata.prel64".len()).any(|w| w == b".rodata.prel64") { prel64 = true; }
        let rela = (ehdr as *mut u8).add(swab_elfxword((*shdr.add(i)).sh_offset) as usize) as *mut Elf64_Rela;
        let numrela = swab_elfxword((*shdr.add(i)).sh_size) as usize / std::mem::size_of::<Elf64_Rela>();
        for j in 0..numrela {
            let mut rinfo = swab_elfxword((*rela.add(j)).r_info);
            if elf64_r_type(rinfo) != R_AARCH64_ABS64 { continue; }
            if prel64 { rinfo ^= (R_AARCH64_ABS64 ^ R_AARCH64_PREL64) as u64; (*rela.add(j)).r_info = swab_elfxword(rinfo); }
            else { fprintf(stderr, b"Unexpected absolute relocations detected in %s\n\0".as_ptr() as *const c_char, *argv.add(2)); close(fd); unlink(*argv.add(1)); exit(libc::EXIT_FAILURE); }
        }
    }
    close(fd);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
