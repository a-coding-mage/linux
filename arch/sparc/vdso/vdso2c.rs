// SPDX-License-Identifier: GPL-2.0-only
/*
 * vdso2c - A vdso image preparation tool
 * Copyright (c) 2014 Andy Lutomirski and others
 *
 * vdso2c requires stripped and unstripped input.  It would be trivial
 * to fully strip the input in here, but, for reasons described below,
 * we need to write a section table.  Doing this is more or less
 * equivalent to dropping all non-allocatable sections, but it's
 * easier to let objcopy handle that instead of doing it ourselves.
 * If we ever need to do something fancier than what objcopy provides,
 * it would be straightforward to add here.
 *
 * We keep a section table for a few reasons:
 *
 * Binutils has issues debugging the vDSO: it reads the section table to
 * find SHT_NOTE; it won't look at PT_NOTE for the in-memory vDSO, which
 * would break build-id if we removed the section table.  Binutils
 * also requires that shstrndx != 0.  See:
 * https://sourceware.org/bugzilla/show_bug.cgi?id=17064
 *
 * elfutils might not look for PT_NOTE if there is a section table at
 * all.  I don't know whether this matters for any practical purpose.
 *
 * For simplicity, rather than hacking up a partial section table, we
 * just write a mostly complete one.  We omit non-dynamic symbols,
 * though, since they're rather large.
 *
 * Once binutils gets fixed, we might be able to drop this for all but
 * the 64-bit vdso, since build-id only works in kernel RPMs, and
 * systems that update to new enough kernel RPMs will likely update
 * binutils in sync.  build-id has never worked for home-built kernel
 * RPMs without manual symlinking, and I suspect that no one ever does
 * that.
 */

/* Copyright (c) 2017 Oracle and/or its affiliates. All rights reserved. */

use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Write;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

// Declarations supplied by the translated vdso2c.h implementation.
extern "C" {
    fn go64(raw_addr: *mut c_void, raw_len: usize, stripped_addr: *mut c_void,
            stripped_len: usize, outfile: *mut c_void, name: *const c_char);
    fn go32(raw_addr: *mut c_void, raw_len: usize, stripped_addr: *mut c_void,
            stripped_len: usize, outfile: *mut c_void, name: *const c_char);
}

static mut OUTFILENAME: *const c_char = ptr::null();

unsafe fn fail(format: *const c_char, _args: *mut c_void) -> ! {
    // The variadic formatting function and stderr operations are external
    // dependencies represented here without inventing their implementations.
    extern "C" {
        fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
        fn vfprintf(stream: *mut c_void, format: *const c_char, args: *mut c_void) -> c_int;
        fn unlink(path: *const c_char) -> c_int;
        fn exit(status: c_int) -> !;
    }
    let _ = (format, fprintf, vfprintf);
    if !OUTFILENAME.is_null() {
        unlink(OUTFILENAME);
    }
    exit(1)
}

// The C source includes vdso2c.h twice with ELF_BITS set to 64 and 32.
// Its declarations and definitions are represented by the external go64/go32
// interfaces above; endian helper macros are preserved as generic operations.

unsafe fn go(raw_addr: *mut c_void, raw_len: usize, stripped_addr: *mut c_void,
             stripped_len: usize, outfile: *mut c_void, name: *const c_char) {
    let class = *(raw_addr as *const u8).add(4);
    const ELFCLASS32: u8 = 1;
    const ELFCLASS64: u8 = 2;
    if class == ELFCLASS64 {
        go64(raw_addr, raw_len, stripped_addr, stripped_len, outfile, name);
    } else if class == ELFCLASS32 {
        go32(raw_addr, raw_len, stripped_addr, stripped_len, outfile, name);
    } else {
        let msg = b"unknown ELF class\0";
        fail(msg.as_ptr() as *const c_char);
    }
}

unsafe fn map_input(_name: *const c_char, addr: *mut *mut c_void, len: *mut usize, _prot: c_int) {
    // mmap/open/lseek/close are external libc operations; preserve the call
    // boundary and ownership semantics expected by the C implementation.
    extern "C" {
        fn open(path: *const c_char, flags: c_int, ...) -> c_int;
        fn lseek(fd: c_int, offset: i64, whence: c_int) -> i64;
        fn mmap(addr: *mut c_void, length: usize, prot: c_int, flags: c_int, fd: c_int, offset: i64) -> *mut c_void;
        fn close(fd: c_int) -> c_int;
        fn err(eval: c_int, format: *const c_char, ... ) -> !;
    }
    const O_RDONLY: c_int = 0;
    const SEEK_END: c_int = 2;
    const PROT_READ: c_int = 1;
    const MAP_PRIVATE: c_int = 2;
    let fd = open(_name, O_RDONLY);
    if fd == -1 { err(1, b"%s\0".as_ptr() as *const c_char, _name); }
    let tmp_len = lseek(fd, 0, SEEK_END);
    if tmp_len == -1 { err(1, b"lseek\0".as_ptr() as *const c_char); }
    *len = tmp_len as usize;
    *addr = mmap(ptr::null_mut(), *len, PROT_READ, MAP_PRIVATE, fd, 0);
    close(fd);
}

unsafe fn main_impl(argc: c_int, argv: *mut *mut c_char) -> c_int {
    extern "C" {
        fn printf(format: *const c_char, ...) -> c_int;
        fn strdup(s: *const c_char) -> *mut c_char;
        fn strlen(s: *const c_char) -> usize;
        fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
        fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
        fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
        fn fopen(path: *const c_char, mode: *const c_char) -> *mut c_void;
        fn fclose(stream: *mut c_void) -> c_int;
        fn munmap(addr: *mut c_void, length: usize) -> c_int;
        fn err(eval: c_int, format: *const c_char, ... ) -> !;
    }
    if argc != 4 {
        printf(b"Usage: vdso2c RAW_INPUT STRIPPED_INPUT OUTPUT\n\0".as_ptr() as *const c_char);
        return 1;
    }
    let raw_name = *argv.add(1);
    let stripped_name = *argv.add(2);
    let output_name = *argv.add(3);
    let name = strdup(output_name);
    let namelen = strlen(name);
    let mut generated_name = name;
    if namelen >= 3 && strcmp(name.add(namelen - 3), b".so\0".as_ptr() as *const c_char) == 0 {
        generated_name = ptr::null_mut();
    } else {
        let mut tmp = strrchr(name, b'/' as c_int);
        if !tmp.is_null() { generated_name = tmp.add(1); }
        tmp = strchr(generated_name, b'.' as c_int);
        if !tmp.is_null() { *tmp = 0; }
        let mut p = generated_name;
        while *p != 0 {
            if *p == b'-' as c_char { *p = b'_' as c_char; }
            p = p.add(1);
        }
    }
    let mut raw_addr: *mut c_void = ptr::null_mut();
    let mut stripped_addr: *mut c_void = ptr::null_mut();
    let mut raw_len = 0usize;
    let mut stripped_len = 0usize;
    map_input(raw_name, &mut raw_addr, &mut raw_len, 1);
    map_input(stripped_name, &mut stripped_addr, &mut stripped_len, 1);
    OUTFILENAME = output_name;
    let outfile = fopen(output_name, b"w\0".as_ptr() as *const c_char);
    if outfile.is_null() { err(1, b"%s\0".as_ptr() as *const c_char, stripped_name); }
    go(raw_addr, raw_len, stripped_addr, stripped_len, outfile, generated_name);
    munmap(raw_addr, raw_len);
    munmap(stripped_addr, stripped_len);
    fclose(outfile);
    0
}

fn main() {
    unsafe { std::process::exit(main_impl(std::env::args().count() as c_int, ptr::null_mut())); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
