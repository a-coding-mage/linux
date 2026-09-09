// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2015 Mentor Graphics Corporation.
 *
 * vdsomunge - Host program which produces a shared object
 * architecturally specified to be usable by both soft- and hard-float
 * programs.
 *
 * The VDSO is built with -msoft-float and uses no floating point arguments
 * or results.  If the soft-float flag is set, this program clears it.
 */

use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

const HOST_ORDER: u8 = if cfg!(target_endian = "little") { 1 } else { 2 };
const EF_ARM_EABI_VER5: u32 = 0x05000000;
const EF_ARM_ABI_FLOAT_SOFT: u32 = 0x200;
const EF_ARM_ABI_FLOAT_HARD: u32 = 0x400;
const ELFCLASS32: u8 = 1;
const ELFDATA2LSB: u8 = 1;
const ET_DYN: u16 = 3;
const EM_ARM: u16 = 40;
const EI_CLASS: usize = 4;
const EI_DATA: usize = 5;
const EI_NIDENT: usize = 16;
const SELFMAG: usize = 4;
const EXIT_FAILURE: c_int = 1;
const EXIT_SUCCESS: c_int = 0;

#[repr(C)]
struct Elf32_Ehdr {
    e_ident: [u8; EI_NIDENT],
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

static mut FAILED: c_int = 0;
static mut ARGV0: *const c_char = ptr::null();
static mut OUTFILE: *const c_char = ptr::null();

extern "C" {
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn vfprintf(stream: *mut c_void, format: *const c_char, ap: *mut c_void) -> c_int;
    fn exit(status: c_int) -> !;
    fn unlink(path: *const c_char) -> c_int;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn fstat(fd: c_int, buf: *mut libc::stat) -> c_int;
    fn mmap(addr: *mut c_void, len: usize, prot: c_int, flags: c_int, fd: c_int, off: libc::off_t) -> *mut c_void;
    fn ftruncate(fd: c_int, length: libc::off_t) -> c_int;
    fn msync(addr: *mut c_void, len: usize, flags: c_int) -> c_int;
    fn strerror(errnum: c_int) -> *const c_char;
    fn __errno_location() -> *mut c_int;
    fn atexit(function: unsafe extern "C" fn());
}

const STDERR_FILENO: c_int = 2;
const O_RDONLY: c_int = 0;
const O_RDWR: c_int = 2;
const O_CREAT: c_int = 64;
const O_TRUNC: c_int = 512;
const S_IRUSR: c_int = 0o400;
const S_IWUSR: c_int = 0o200;
const PROT_READ: c_int = 1;
const PROT_WRITE: c_int = 2;
const MAP_PRIVATE: c_int = 2;
const MAP_SHARED: c_int = 1;
const MAP_FAILED: *mut c_void = !0 as *mut c_void;
const MS_SYNC: c_int = 4;

fn swab16(x: u16) -> u16 { ((x & 0x00ff) << 8) | ((x & 0xff00) >> 8) }
fn swab32(x: u32) -> u32 {
    ((x & 0x000000ff) << 24) | ((x & 0x0000ff00) << 8) |
    ((x & 0x00ff0000) >> 8) | ((x & 0xff000000) >> 24)
}
fn read_elf_word(word: u32, swap: bool) -> u32 { if swap { swab32(word) } else { word } }
fn read_elf_half(half: u16, swap: bool) -> u16 { if swap { swab16(half) } else { half } }
unsafe fn write_elf_word(val: u32, dst: *mut u32, swap: bool) { *dst = if swap { swab32(val) } else { val }; }

unsafe fn fail(fmt: *const c_char, arg: *const c_char) -> ! {
    FAILED = 1;
    fprintf(STDERR_FILENO as *mut c_void, b"%s: \0".as_ptr() as *const c_char, ARGV0);
    fprintf(STDERR_FILENO as *mut c_void, fmt, arg);
    exit(EXIT_FAILURE);
}

unsafe extern "C" fn cleanup() {
    if FAILED != 0 && !OUTFILE.is_null() { unlink(OUTFILE); }
}

unsafe fn cstr(p: *const c_char) -> &'static CStr { CStr::from_ptr(p) }

pub unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    atexit(cleanup);
    ARGV0 = *argv;
    if argc != 3 { fail(b"Usage: %s [infile] [outfile]\n\0".as_ptr() as *const c_char, *argv); }
    let infile = *argv.add(1);
    OUTFILE = *argv.add(2);
    let infd = open(infile, O_RDONLY);
    if infd < 0 { fail(b"Cannot open %s\n\0".as_ptr() as *const c_char, infile); }
    let mut stat: libc::stat = mem::zeroed();
    if fstat(infd, &mut stat) != 0 { fail(b"Failed stat for %s\n\0".as_ptr() as *const c_char, infile); }
    let size = stat.st_size as usize;
    let inbuf = mmap(ptr::null_mut(), size, PROT_READ, MAP_PRIVATE, infd, 0);
    if inbuf == MAP_FAILED { fail(b"Failed to map %s\n\0".as_ptr() as *const c_char, infile); }
    close(infd);
    let inhdr = inbuf as *const Elf32_Ehdr;
    if (*inhdr).e_ident[..SELFMAG] != [0x7f, b'E', b'L', b'F'] { fail(b"Not an ELF file\n\0".as_ptr() as *const c_char, ptr::null()); }
    if (*inhdr).e_ident[EI_CLASS] != ELFCLASS32 { fail(b"Unsupported ELF class\n\0".as_ptr() as *const c_char, ptr::null()); }
    let swap = (*inhdr).e_ident[EI_DATA] != HOST_ORDER;
    if read_elf_half((*inhdr).e_type, swap) != ET_DYN { fail(b"Not a shared object\n\0".as_ptr() as *const c_char, ptr::null()); }
    if read_elf_half((*inhdr).e_machine, swap) != EM_ARM { fail(b"Unsupported architecture\n\0".as_ptr() as *const c_char, ptr::null()); }
    let mut e_flags = read_elf_word((*inhdr).e_flags, swap);
    if e_flags & 0xff000000 != EF_ARM_EABI_VER5 { fail(b"Unsupported EABI version\n\0".as_ptr() as *const c_char, ptr::null()); }
    if e_flags & EF_ARM_ABI_FLOAT_HARD != 0 { fail(b"Unexpected hard-float flag set in e_flags\n\0".as_ptr() as *const c_char, ptr::null()); }
    let clear_soft_float = e_flags & EF_ARM_ABI_FLOAT_SOFT != 0;
    let outfd = open(OUTFILE, O_RDWR | O_CREAT | O_TRUNC, S_IRUSR | S_IWUSR);
    if outfd < 0 { fail(b"Cannot open %s\n\0".as_ptr() as *const c_char, OUTFILE); }
    if ftruncate(outfd, stat.st_size) != 0 { fail(b"Cannot truncate %s\n\0".as_ptr() as *const c_char, OUTFILE); }
    let outbuf = mmap(ptr::null_mut(), size, PROT_READ | PROT_WRITE, MAP_SHARED, outfd, 0);
    if outbuf == MAP_FAILED { fail(b"Failed to map %s\n\0".as_ptr() as *const c_char, OUTFILE); }
    close(outfd);
    ptr::copy_nonoverlapping(inbuf as *const u8, outbuf as *mut u8, size);
    if clear_soft_float {
        e_flags &= !EF_ARM_ABI_FLOAT_SOFT;
        write_elf_word(e_flags, &mut (*(outbuf as *mut Elf32_Ehdr)).e_flags, swap);
    }
    if msync(outbuf, size, MS_SYNC) != 0 { fail(b"Failed to sync %s\n\0".as_ptr() as *const c_char, OUTFILE); }
    EXIT_SUCCESS
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
