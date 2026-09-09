// SPDX-License-Identifier: GPL-2.0
/*
 * Makes a tree bootable image for IBM Evaluation boards.
 * Basically, just take a zImage, skip the ELF header, and stuff
 * a 32 byte header on the front.
 *
 * We use htonl, which is a network macro, to make sure we're doing
 * The Right Thing on an LE machine.  It's non-obvious, but it should
 * work on anything BSD'ish.
 */

use std::ffi::CString;
use std::io::{self, Write};
use std::mem::{size_of, MaybeUninit};
use std::os::raw::{c_char, c_int, c_uint, c_void};
use std::ptr;

const IMGBLK: usize = 512;
static mut TMPBUF: [c_uint; IMGBLK / size_of::<c_uint>()] =
    [0; IMGBLK / size_of::<c_uint>()];

#[repr(C)]
struct BootBlock {
    bb_magic: u32,
    bb_dest: u32,
    bb_num_512blocks: u32,
    bb_debug_flag: u32,
    bb_entry_point: u32,
    bb_checksum: u32,
    reserved: [u32; 2],
}

#[repr(C)]
struct Stat {
    _data: [u8; 512],
}

extern "C" {
    fn stat(path: *const c_char, buf: *mut Stat) -> c_int;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn lseek(fd: c_int, offset: i64, whence: c_int) -> i64;
    fn htonl(hostlong: u32) -> u32;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> u64;
}

const O_RDONLY: c_int = 0;
const O_RDWR: c_int = 2;
const O_CREAT: c_int = 64;
const O_TRUNC: c_int = 512;
const SEEK_SET: c_int = 0;

fn c_string(s: &str) -> CString {
    CString::new(s).unwrap()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 5 {
        let _ = writeln!(io::stderr(), "usage: {} <zImage-file> <boot-image> <load address> <entry point>", args[0]);
        std::process::exit(1);
    }

    let input = c_string(&args[1]);
    let output = c_string(&args[2]);
    let mut st = MaybeUninit::<Stat>::uninit();
    unsafe {
        if stat(input.as_ptr(), st.as_mut_ptr()) < 0 {
            let _ = writeln!(io::stderr(), "stat: {}", io::Error::last_os_error());
            std::process::exit(2);
        }
    }
    // The source uses st.st_size; the platform stat layout is supplied externally.
    let st_size = unsafe { *(st.as_ptr().cast::<i64>()) };
    let mut nblks = ((st_size as usize + IMGBLK) / IMGBLK) as c_int;

    let mut bt = BootBlock {
        bb_magic: unsafe { htonl(0x0052504f) },
        bb_dest: 0,
        bb_num_512blocks: 0,
        bb_debug_flag: 0,
        bb_entry_point: 0,
        bb_checksum: 0,
        reserved: [0, 0],
    };
    let mut end = ptr::null_mut();
    bt.bb_dest = unsafe { htonl(strtoul(c_string(&args[3]).as_ptr(), &mut end, 0) as u32) };
    bt.bb_entry_point = unsafe { htonl(strtoul(c_string(&args[4]).as_ptr(), &mut end, 0) as u32) };
    bt.bb_num_512blocks = unsafe { htonl(nblks as u32) };

    let in_fd = unsafe { open(input.as_ptr(), O_RDONLY) };
    if in_fd < 0 {
        let _ = writeln!(io::stderr(), "zImage open: {}", io::Error::last_os_error());
        std::process::exit(3);
    }
    let out_fd = unsafe { open(output.as_ptr(), O_RDWR | O_CREAT | O_TRUNC, 0o666) };
    if out_fd < 0 {
        let _ = writeln!(io::stderr(), "bootfile open: {}", io::Error::last_os_error());
        std::process::exit(3);
    }

    let mut cksum: c_uint = 0;
    let words = size_of::<BootBlock>() / size_of::<c_uint>();
    let bp = &bt as *const BootBlock as *const c_uint;
    for i in 0..words { cksum = cksum.wrapping_add(unsafe { *bp.add(i) }); }

    unsafe {
        if read(in_fd, TMPBUF.as_mut_ptr().cast(), size_of_val(&TMPBUF)) != size_of_val(&TMPBUF) as isize {
            let _ = writeln!(io::stderr(), "{} is too small to be an ELF image", args[1]); std::process::exit(4);
        }
        if TMPBUF[0] != htonl(0x7f454c46) { let _ = writeln!(io::stderr(), "{} is not an ELF image", args[1]); std::process::exit(4); }
        if lseek(in_fd, (64 * 1024) as i64, SEEK_SET) < 0 { let _ = writeln!(io::stderr(), "{} failed to seek in ELF image", args[1]); std::process::exit(4); }
    }
    nblks -= (64 * 1024 / IMGBLK) as c_int;
    unsafe {
        if write(out_fd, (&bt as *const BootBlock).cast(), size_of::<BootBlock>()) != size_of::<BootBlock>() as isize { std::process::exit(5); }
        while nblks > 0 { nblks -= 1; if read(in_fd, TMPBUF.as_mut_ptr().cast(), size_of_val(&TMPBUF)) < 0 { std::process::exit(5); } for i in 0..TMPBUF.len() { cksum = cksum.wrapping_add(TMPBUF[i]); } if write(out_fd, TMPBUF.as_ptr().cast(), size_of_val(&TMPBUF)) != size_of_val(&TMPBUF) as isize { std::process::exit(5); } }
        bt.bb_checksum = htonl(cksum);
        if lseek(out_fd, 0, SEEK_SET) < 0 || write(out_fd, (&bt as *const BootBlock).cast(), size_of::<BootBlock>()) != size_of::<BootBlock>() as isize { std::process::exit(1); }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
