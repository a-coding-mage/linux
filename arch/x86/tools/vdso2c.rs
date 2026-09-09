// SPDX-License-Identifier: GPL-2.0-only
/*
 * vdso2c - A vdso image preparation tool
 * Copyright (c) 2014 Andy Lutomirski and others
 *
 * This is a source-level Rust translation of the C implementation.
 */

use std::ffi::{c_char, c_int, c_void, CStr};
use std::mem;
use std::ptr;

// Declarations supplied by vdso2c.h and the platform headers.
#[repr(C)]
struct Elf64_Ehdr {
    e_ident: [u8; 16],
    _rest: [u8; 48],
}

extern "C" {
    fn go64(raw_addr: *mut c_void, raw_len: usize, stripped_addr: *mut c_void,
            stripped_len: usize, outfile: *mut c_void, name: *const c_char);
    fn go32(raw_addr: *mut c_void, raw_len: usize, stripped_addr: *mut c_void,
            stripped_len: usize, outfile: *mut c_void, name: *const c_char);
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut c_void;
    fn fclose(stream: *mut c_void) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...);
    fn printf(format: *const c_char, ...);
    fn stderr() -> *mut c_void;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn unlink(path: *const c_char) -> c_int;
    fn exit(status: c_int) -> !;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn lseek(fd: c_int, offset: isize, whence: c_int) -> isize;
    fn mmap(addr: *mut c_void, len: usize, prot: c_int, flags: c_int,
            fd: c_int, offset: isize) -> *mut c_void;
    fn munmap(addr: *mut c_void, len: usize) -> c_int;
    fn close(fd: c_int) -> c_int;
}

static mut OUTFILENAME: *const c_char = ptr::null();

#[repr(C)]
struct VdsoSym {
    name: *const c_char,
    export_: bool,
}

static REQUIRED_SYMS: &[VdsoSym] = &[
    VdsoSym { name: b"__kernel_vsyscall\0".as_ptr() as *const c_char, export_: true },
    VdsoSym { name: b"__kernel_sigreturn\0".as_ptr() as *const c_char, export_: true },
    VdsoSym { name: b"__kernel_rt_sigreturn\0".as_ptr() as *const c_char, export_: true },
    VdsoSym { name: b"int80_landing_pad\0".as_ptr() as *const c_char, export_: true },
    VdsoSym { name: b"vdso32_rt_sigreturn_landing_pad\0".as_ptr() as *const c_char, export_: true },
    VdsoSym { name: b"vdso32_sigreturn_landing_pad\0".as_ptr() as *const c_char, export_: true },
    VdsoSym { name: b"__futex_list64_try_unlock_cs_start\0".as_ptr() as *const c_char, export_: true },
    VdsoSym { name: b"__futex_list64_try_unlock_cs_end\0".as_ptr() as *const c_char, export_: true },
    VdsoSym { name: b"__futex_list32_try_unlock_cs_start\0".as_ptr() as *const c_char, export_: true },
    VdsoSym { name: b"__futex_list32_try_unlock_cs_end\0".as_ptr() as *const c_char, export_: true },
];

unsafe fn fail(format: *const c_char) -> ! {
    fprintf(stderr(), b"Error: \0".as_ptr() as *const c_char);
    fprintf(stderr(), format);
    if !OUTFILENAME.is_null() { unlink(OUTFILENAME); }
    exit(1)
}

unsafe fn go(raw_addr: *mut c_void, raw_len: usize, stripped_addr: *mut c_void,
             stripped_len: usize, outfile: *mut c_void, name: *const c_char) {
    let hdr = raw_addr as *const Elf64_Ehdr;
    const EI_CLASS: usize = 4;
    const ELFCLASS32: u8 = 1;
    const ELFCLASS64: u8 = 2;
    match (*hdr).e_ident[EI_CLASS] {
        ELFCLASS64 => go64(raw_addr, raw_len, stripped_addr, stripped_len, outfile, name),
        ELFCLASS32 => go32(raw_addr, raw_len, stripped_addr, stripped_len, outfile, name),
        _ => fail(b"unknown ELF class\n\0".as_ptr() as *const c_char),
    }
}

unsafe fn map_input(name: *const c_char, addr: *mut *mut c_void, len: *mut usize, prot: c_int) {
    const O_RDONLY: c_int = 0;
    const SEEK_END: c_int = 2;
    const MAP_PRIVATE: c_int = 2;
    let fd = open(name, O_RDONLY);
    if fd == -1 { exit(1); }
    let tmp_len = lseek(fd, 0, SEEK_END);
    if tmp_len == -1 { exit(1); }
    *len = tmp_len as usize;
    *addr = mmap(ptr::null_mut(), tmp_len as usize, prot, MAP_PRIVATE, fd, 0);
    if *addr as isize == -1 { exit(1); }
    close(fd);
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    if argc != 4 {
        printf(b"Usage: vdso2c RAW_INPUT STRIPPED_INPUT OUTPUT\n\0".as_ptr() as *const c_char);
        return 1;
    }
    let mut raw_len = 0usize;
    let mut stripped_len = 0usize;
    let mut raw_addr = ptr::null_mut();
    let mut stripped_addr = ptr::null_mut();
    let output = *argv.add(3);
    let mut name = strdup(output);
    let namelen = strlen(name);
    if namelen >= 3 && strcmp(name.add(namelen - 3), b".so\0".as_ptr() as *const c_char) == 0 {
        name = ptr::null_mut();
    } else {
        let mut tmp = strrchr(name, b'/' as c_int);
        if !tmp.is_null() { name = tmp.add(1); }
        tmp = strchr(name, b'.' as c_int);
        if !tmp.is_null() { *tmp = 0; }
        let mut p = name;
        while *p != 0 {
            if *p == b'-' as c_char { *p = b'_' as c_char; }
            p = p.add(1);
        }
    }
    OUTFILENAME = output;
    map_input(*argv.add(1), &mut raw_addr, &mut raw_len, 1);
    map_input(*argv.add(2), &mut stripped_addr, &mut stripped_len, 1);
    let outfile = fopen(output, b"w\0".as_ptr() as *const c_char);
    if outfile.is_null() { exit(1); }
    go(raw_addr, raw_len, stripped_addr, stripped_len, outfile, name);
    munmap(raw_addr, raw_len);
    munmap(stripped_addr, stripped_len);
    fclose(outfile);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
