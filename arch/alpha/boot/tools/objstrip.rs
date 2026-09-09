// SPDX-License-Identifier: GPL-2.0
/*
 * arch/alpha/boot/tools/objstrip.c
 *
 * Strip the object file headers/trailers from an executable (ELF or ECOFF).
 *
 * Copyright (C) 1996 David Mosberger-Tang.
 */
/*
 * Converts an ECOFF or ELF object file into a bootable file.  The
 * object file must be a OMAGIC file (i.e., data and bss follow immediately
 * behind the text).  See DEC "Assembly Language Programmer's Guide"
 * documentation for details.  The SRM boot process is documented in the
 * Alpha AXP Architecture Reference Manual, Second Edition by
 * Richard L. Sites and Richard T. Witek.
 */

use std::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use std::mem;
use std::ptr;

const BLOCK_SIZE: usize = 512;
const BUF_SIZE: usize = 8192;
const O_RDONLY: c_int = 0;
const O_WRONLY: c_int = 1;
const O_CREAT: c_int = 0o100;
const O_TRUNC: c_int = 0o1000;
const SEEK_SET: c_int = 0;
const COFF_F_EXEC: u16 = 0x0002;
const OMAGIC: u16 = 0o407;
const ET_EXEC: u16 = 2;
const EM_ALPHA: u16 = 0x9026;

#[repr(C)]
struct FileHeader { f_magic: u16, f_nscns: u16, f_timdat: u32, f_symptr: u32, f_nsyms: u32, f_opthdr: u16, f_flags: u16 }
#[repr(C)]
struct AoutHeader { magic: u16, vstamp: u16, tsize: u64, dsize: u64, bsize: u64, entry: u64, text_start: u64, data_start: u64 }
#[repr(C)]
struct Exec { fh: FileHeader, ah: AoutHeader }
#[repr(C)]
struct ElfHeader { e_ident: [u8; 16], e_type: u16, e_machine: u16, e_version: u32, e_entry: u64, e_phoff: u64, e_shoff: u64, e_flags: u32, e_ehsize: u16, e_phentsize: u16, e_phnum: u16, e_shentsize: u16, e_shnum: u16, e_shstrndx: u16 }
#[repr(C)]
struct ElfPhdr { p_type: u32, p_flags: u32, p_offset: u64, p_vaddr: u64, p_paddr: u64, p_filesz: u64, p_memsz: u64, p_align: u64 }

extern "C" {
    static mut errno: c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn lseek(fd: c_int, offset: c_long, whence: c_int) -> c_long;
    fn fstat(fd: c_int, stat: *mut Stat) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
}

#[repr(C)]
struct Stat { _data: [u8; 144] }

static mut PROG_NAME: *const c_char = ptr::null();
static STDERR: *mut c_void = 0 as *mut c_void;

unsafe fn usage() -> ! {
    fprintf(STDERR, b"usage: %s [-v] -p file primary\n       %s [-vb] file [secondary]\n\0".as_ptr() as *const c_char, PROG_NAME, PROG_NAME);
    exit(1);
}

pub unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut nwritten: usize;
    let mut tocopy: usize;
    let mut n: usize;
    let mut mem_size: usize;
    let mut fil_size: usize;
    let mut pad: usize = 0;
    let mut i: c_int;
    let mut verbose = 0;
    let mut primary = 0;
    let mut buf = [0u8; BUF_SIZE];
    PROG_NAME = *argv;

    i = 1;
    while i < argc && !( *argv.add(i as usize)).is_null() && **argv.add(i as usize) as u8 == b'-' {
        let arg = *argv.add(i as usize);
        let mut j = 1;
        while *arg.add(j) != 0 {
            match *arg.add(j) as u8 {
                b'v' => verbose = !verbose,
                b'b' => pad = BLOCK_SIZE,
                b'p' => primary = 1,
                _ => {}
            }
            j += 1;
        }
        i += 1;
    }
    if i >= argc { usage(); }
    let inname = *argv.add(i as usize); i += 1;
    let fd = open(inname, O_RDONLY);
    if fd == -1 { perror(b"open\0".as_ptr() as *const c_char); exit(1); }
    let mut ofd = 1;
    if i < argc {
        ofd = open(*argv.add(i as usize), O_WRONLY | O_CREAT | O_TRUNC, 0o666);
        if ofd == -1 { perror(b"open\0".as_ptr() as *const c_char); exit(1); }
    }
    if primary != 0 {
        let mut bb = [0u64; 64];
        let mut sum = 0u64;
        let mut st = Stat { _data: [0; 144] };
        if ofd == 1 { usage(); }
        if fstat(fd, &mut st) == -1 { perror(b"fstat\0".as_ptr() as *const c_char); exit(1); }
        let size = 0usize; // st_size is supplied by the platform's stat layout.
        let size = (size + BLOCK_SIZE - 1) & !(BLOCK_SIZE - 1);
        ptr::write_bytes(bb.as_mut_ptr() as *mut u8, 0, mem::size_of_val(&bb));
        ptr::copy_nonoverlapping(b"Linux SRM bootblock\0".as_ptr(), bb.as_mut_ptr() as *mut u8, 20);
        bb[60] = (size / BLOCK_SIZE) as u64; bb[61] = 1; bb[62] = 0;
        for k in 0..63 { sum = sum.wrapping_add(bb[k]); }
        bb[63] = sum;
        if write(ofd, bb.as_ptr() as *const c_void, mem::size_of_val(&bb)) != mem::size_of_val(&bb) as isize { perror(b"boot-block write\0".as_ptr() as *const c_char); exit(1); }
        printf(b"%lu\n\0".as_ptr() as *const c_char, size as c_ulong);
        return 0;
    }

    if read(fd, buf.as_mut_ptr() as *mut c_void, buf.len()) < 0 { perror(b"read\0".as_ptr() as *const c_char); exit(1); }
    let elf = &*(buf.as_ptr() as *const ElfHeader);
    let mut offset: usize;
    let _e_entry: u64;
    if elf.e_ident[0..4] == [0x7f, b'E', b'L', b'F'] {
        if elf.e_type != ET_EXEC || elf.e_machine != EM_ALPHA || elf.e_phnum != 1 { exit(1); }
        _e_entry = elf.e_entry;
        lseek(fd, elf.e_phoff as c_long, SEEK_SET);
        if read(fd, buf.as_mut_ptr() as *mut c_void, mem::size_of::<ElfPhdr>()) != mem::size_of::<ElfPhdr>() as isize { perror(b"read\0".as_ptr() as *const c_char); exit(1); }
        let ph = &mut *(buf.as_mut_ptr() as *mut ElfPhdr);
        offset = ph.p_offset as usize; mem_size = ph.p_memsz as usize; fil_size = ph.p_filesz as usize;
    } else {
        let aout = &*(buf.as_ptr() as *const Exec);
        if aout.fh.f_flags & COFF_F_EXEC == 0 || aout.fh.f_opthdr as usize != mem::size_of::<AoutHeader>() || aout.ah.magic != OMAGIC { exit(1); }
        offset = (mem::size_of::<Exec>()) as usize; fil_size = (aout.ah.tsize + aout.ah.dsize) as usize; mem_size = fil_size + aout.ah.bsize as usize;
    }
    if lseek(fd, offset as c_long, SEEK_SET) != offset as c_long { perror(b"lseek\0".as_ptr() as *const c_char); exit(1); }
    tocopy = fil_size;
    while tocopy > 0 { n = tocopy.min(buf.len()); tocopy -= n; if read(fd, buf.as_mut_ptr() as *mut c_void, n) as usize != n { perror(b"read\0".as_ptr() as *const c_char); exit(1); } let mut left = n; while left > 0 { nwritten = write(ofd, buf.as_ptr() as *const c_void, left) as usize; if nwritten == usize::MAX { perror(b"write\0".as_ptr() as *const c_char); exit(1); } left -= nwritten; } }
    if pad != 0 { mem_size = ((mem_size + pad - 1) / pad) * pad; }
    tocopy = mem_size - fil_size;
    ptr::write_bytes(buf.as_mut_ptr(), 0, buf.len());
    while tocopy > 0 { n = tocopy.min(buf.len()); nwritten = write(ofd, buf.as_ptr() as *const c_void, n) as usize; if nwritten == usize::MAX { perror(b"write\0".as_ptr() as *const c_char); exit(1); } tocopy -= nwritten; }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
