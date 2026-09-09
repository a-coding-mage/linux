// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Program to hack in a PT_NOTE program header entry in an ELF file.
 * This is needed for OF on RS/6000s to load an image correctly.
 * Note that OF needs a program header entry for the note, not an
 * ELF section.
 *
 * Copyright 2000 Paul Mackerras.
 *
 * Adapted for 64 bit little endian images by Andrew Tauferner.
 *
 * Usage: addnote zImage
 */

use std::ffi::{c_char, c_int, c_void, CStr};

unsafe extern "C" {
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn lseek(fd: c_int, offset: i64, whence: c_int) -> i64;
    fn close(fd: c_int) -> c_int;
    fn perror(s: *const c_char);
    fn fprintf(stream: *mut c_void, format: *const c_char, ...);
    fn exit(status: c_int) -> !;
    static mut stderr: *mut c_void;
}

/* CHRP note section */
static ARCH: &[u8] = b"PowerPC\0";

const N_DESCR: usize = 6;
static mut DESCR: [u32; N_DESCR] = [
    0xffffffff, // real-mode = true
    0x02000000, // real-base, i.e. where we expect OF to be
    0xffffffff, // real-size
    0xffffffff, // virt-base
    0xffffffff, // virt-size
    0x4000,     // load-base
];

/* RPA note section */
static RPANAME: &[u8] = b"IBM,RPA-Client-Config\0";

/*
 * Note: setting ignore_my_client_config *should* mean that OF ignores
 * all the other fields, but there is a firmware bug which means that
 * it looks at the splpar field at least.  So these values need to be
 * reasonable.
 */
const N_RPA_DESCR: usize = 8;
static mut RPANOTE: [u32; N_RPA_DESCR] = [
    0, 64, 0, 40, 1, u32::MAX, 0, 1,
];

static mut BUF: [u8; 1024] = [0; 1024];
const ELFDATA2LSB: i32 = 1;
const ELFDATA2MSB: i32 = 2;
static mut E_DATA: i32 = ELFDATA2MSB;
const ELFCLASS32: i32 = 1;
const ELFCLASS64: i32 = 2;
static mut E_CLASS: i32 = ELFCLASS32;

fn roundup(len: usize) -> usize { (len + 3) & !3 }

unsafe fn get16be(off: usize) -> u16 { ((BUF[off] as u16) << 8) + BUF[off + 1] as u16 }
unsafe fn get32be(off: usize) -> u32 { ((get16be(off) as u32) << 16) + get16be(off + 2) as u32 }
unsafe fn get64be(off: usize) -> u64 { ((get32be(off) as u64) << 32) + get32be(off + 4) as u64 }
unsafe fn put16be(off: usize, v: u64) { BUF[off] = ((v >> 8) & 0xff) as u8; BUF[off + 1] = (v & 0xff) as u8; }
unsafe fn put32be(off: usize, v: u64) { put16be(off, v >> 16); put16be(off + 2, v); }
unsafe fn put64be(off: usize, v: u64) { put32be(off, v >> 32); put32be(off + 4, v); }
unsafe fn get16le(off: usize) -> u16 { BUF[off] as u16 + ((BUF[off + 1] as u16) << 8) }
unsafe fn get32le(off: usize) -> u32 { get16le(off) as u32 + ((get16le(off + 2) as u32) << 16) }
unsafe fn get64le(off: usize) -> u64 { get32le(off) as u64 + ((get32le(off + 4) as u64) << 32) }
unsafe fn put16le(off: usize, v: u64) { BUF[off] = (v & 0xff) as u8; BUF[off + 1] = ((v >> 8) & 0xff) as u8; }
unsafe fn put32le(off: usize, v: u64) { put16le(off, v); put16le(off + 2, v >> 16); }
unsafe fn put64le(off: usize, v: u64) { put32le(off, v); put32le(off + 4, v >> 32); }
unsafe fn get16(off: usize) -> u16 { if E_DATA == ELFDATA2MSB { get16be(off) } else { get16le(off) } }
unsafe fn get32(off: usize) -> u32 { if E_DATA == ELFDATA2MSB { get32be(off) } else { get32le(off) } }
unsafe fn get64(off: usize) -> u64 { if E_DATA == ELFDATA2MSB { get64be(off) } else { get64le(off) } }
unsafe fn put16(off: usize, v: u64) { if E_DATA == ELFDATA2MSB { put16be(off, v) } else { put16le(off, v) } }
unsafe fn put32(off: usize, v: u64) { if E_DATA == ELFDATA2MSB { put32be(off, v) } else { put32le(off, v) } }
unsafe fn put64(off: usize, v: u64) { if E_DATA == ELFDATA2MSB { put64be(off, v) } else { put64le(off, v) } }

const PT_NOTE: u32 = 4;
const O_RDWR: c_int = 2;
const SEEK_SET: c_int = 0;

unsafe fn die(msg: *const c_char) -> ! { perror(msg); exit(1) }

#[no_mangle]
pub unsafe extern "C" fn main(ac: c_int, av: *mut *mut c_char) -> c_int {
    if ac != 2 {
        fprintf(stderr, b"Usage: %s elf-file\n\0".as_ptr() as *const c_char, *av);
        exit(1);
    }
    let name = *av.add(1);
    let fd = open(name, O_RDWR);
    if fd < 0 { die(name); }

    let nnote = 12 + roundup(ARCH.len()) + std::mem::size_of::<[u32; N_DESCR]>();
    let nnote2 = 12 + roundup(RPANAME.len()) + std::mem::size_of::<[u32; N_RPA_DESCR]>();
    let n = read(fd, BUF.as_mut_ptr() as *mut c_void, BUF.len());
    if n < 0 { die(b"read\0".as_ptr() as *const c_char); }
    let n = n as usize;
    if BUF[0..4] != [0x7f, b'E', b'L', b'F'] { goto_notelf(name); }
    E_CLASS = BUF[4] as i32;
    if E_CLASS != ELFCLASS32 && E_CLASS != ELFCLASS64 { goto_notelf(name); }
    E_DATA = BUF[5] as i32;
    if E_DATA != ELFDATA2MSB && E_DATA != ELFDATA2LSB { goto_notelf(name); }
    let ehsize = if E_CLASS == ELFCLASS32 { 52 } else { 64 };
    if n < ehsize { goto_notelf(name); }
    let mut ph = if E_CLASS == ELFCLASS32 { get32(28) as usize } else { get64(32) as usize };
    let ps = get16(if E_CLASS == ELFCLASS32 { 42 } else { 54 }) as usize;
    let np = get16(if E_CLASS == ELFCLASS32 { 44 } else { 56 }) as usize;
    let ph_hsize = if E_CLASS == ELFCLASS32 { 32 } else { 56 };
    if ph < ehsize || ps < ph_hsize || np < 1 { goto_notelf(name); }
    if ph + (np + 2) * ps + nnote + nnote2 > n { goto_nospace(name); }
    for _ in 0..np {
        if get32(ph) == PT_NOTE { fprintf(stderr, b"%s already has a note entry\n\0".as_ptr() as *const c_char, name); exit(0); }
        ph += ps;
    }
    for i in 0..(2 * ps + nnote + nnote2) { if BUF[ph + i] != 0 { goto_nospace(name); } }
    let mut ns = ph + 2 * ps;
    put32(ph, PT_NOTE as u64);
    if E_CLASS == ELFCLASS32 { put32(ph + 4, ns as u64); put32(ph + 16, nnote as u64); } else { put64(ph + 8, ns as u64); put64(ph + 32, nnote as u64); }
    put32(ns, ARCH.len() as u64); put32(ns + 4, (N_DESCR * 4) as u64); put32(ns + 8, 0x1275);
    BUF[ns + 12..ns + 12 + ARCH.len()].copy_from_slice(ARCH); ns += 12 + ARCH.len();
    for i in 0..N_DESCR { put32be(ns, DESCR[i] as u64); ns += 4; }
    ph += ps; put32(ph, PT_NOTE as u64);
    if E_CLASS == ELFCLASS32 { put32(ph + 4, ns as u64); put32(ph + 16, nnote as u64); } else { put64(ph + 8, ns as u64); put64(ph + 32, nnote2 as u64); }
    put32(ns, RPANAME.len() as u64); put32(ns + 4, std::mem::size_of_val(&RPANOTE) as u64); put32(ns + 8, 0x12759999);
    BUF[ns + 12..ns + 12 + RPANAME.len()].copy_from_slice(RPANAME); ns += 12 + roundup(RPANAME.len());
    for i in 0..N_RPA_DESCR { put32be(ns, RPANOTE[i] as u64); ns += 4; }
    put16(if E_CLASS == ELFCLASS32 { 44 } else { 56 }, (np + 2) as u64);
    if lseek(fd, 0, SEEK_SET) < 0 { die(b"lseek\0".as_ptr() as *const c_char); }
    let written = write(fd, BUF.as_ptr() as *const c_void, n);
    if written < 0 { die(b"write\0".as_ptr() as *const c_char); }
    if written < n as isize { fprintf(stderr, b"%s: write truncated\n\0".as_ptr() as *const c_char, name); exit(1); }
    close(fd); exit(0)
}

unsafe fn goto_notelf(name: *mut c_char) -> ! {
    fprintf(stderr, b"%s does not appear to be an ELF file\n\0".as_ptr() as *const c_char, name); exit(1)
}
unsafe fn goto_nospace(name: *mut c_char) -> ! {
    fprintf(stderr, b"sorry, I can't find space in %s to put the note\n\0".as_ptr() as *const c_char, name); exit(1)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
