// SPDX-License-Identifier: GPL-2.0-or-later
/*
   Simple utility to make a single-image install kernel with initial ramdisk
   for Sparc tftpbooting without need to set up nfs.

   Copyright (C) 1996,1997 Jakub Jelinek (jj@sunsite.mff.cuni.cz)
   Pete Zaitcev <zaitcev@yahoo.com> endian fixes for cross-compiles, 2000.
   Copyright (C) 2011 Sam Ravnborg <sam@ravnborg.org>
 */

// C headers and platform declarations are supplied by the surrounding build.

const AOUT_TEXT_OFFSET: libc::off_t = 32;
static mut IS64BIT: libc::c_int = 0;

/* align to power-of-two size */
unsafe fn align(n: libc::c_int) -> libc::c_int {
    if IS64BIT != 0 { (n + 0x1fff) & !0x1fff } else { (n + 0xfff) & !0xfff }
}

/* read two bytes as big endian */
unsafe fn ld2(p: *mut libc::c_char) -> libc::c_ushort {
    (((*p as libc::c_ushort) << 8) | *((p.add(1)) as *mut libc::c_ushort))
}

/* save 4 bytes as big endian */
unsafe fn st4(p: *mut libc::c_char, x: libc::c_uint) {
    *p = (x >> 24) as libc::c_char;
    *p.add(1) = (x >> 16) as libc::c_char;
    *p.add(2) = (x >> 8) as libc::c_char;
    *p.add(3) = x as libc::c_char;
}

unsafe fn die(str_: *const libc::c_char) -> ! {
    libc::perror(str_);
    libc::exit(1);
}

unsafe fn usage() -> ! {
    libc::fputs(b"Usage: piggyback bits vmlinux.aout System.map fs_img.gz\n\0".as_ptr() as *const libc::c_char, libc::stderr);
    libc::fputs(b"\tKernel image will be modified in place.\n\0".as_ptr() as *const libc::c_char, libc::stderr);
    libc::exit(1);
}

unsafe fn start_line(line: *const libc::c_char) -> libc::c_int {
    if libc::strcmp(line.add(10), b" _start\n\0".as_ptr() as *const libc::c_char) == 0 || libc::strcmp(line.add(18), b" _start\n\0".as_ptr() as *const libc::c_char) == 0 { 1 } else { 0 }
}

unsafe fn end_line(line: *const libc::c_char) -> libc::c_int {
    if libc::strcmp(line.add(10), b" _end\n\0".as_ptr() as *const libc::c_char) == 0 || libc::strcmp(line.add(18), b" _end\n\0".as_ptr() as *const libc::c_char) == 0 { 1 } else { 0 }
}

unsafe fn get_start_end(filename: *const libc::c_char, start: *mut libc::c_uint, end: *mut libc::c_uint) -> libc::c_int {
    let mut buffer = [0 as libc::c_char; 1024];
    *start = 0; *end = 0;
    let map = libc::fopen(filename, b"r\0".as_ptr() as *const libc::c_char);
    if map.is_null() { die(filename); }
    while !libc::fgets(buffer.as_mut_ptr(), 1024, map).is_null() {
        if start_line(buffer.as_ptr()) != 0 { *start = libc::strtoul(buffer.as_ptr(), core::ptr::null_mut(), 16) as libc::c_uint; }
        else if end_line(buffer.as_ptr()) != 0 { *end = libc::strtoul(buffer.as_ptr(), core::ptr::null_mut(), 16) as libc::c_uint; }
    }
    libc::fclose(map);
    if *start == 0 || *end == 0 { 0 } else { 1 }
}

const LOOKBACK: libc::off_t = 128 * 4;
const BUFSIZE: libc::c_int = 1024;

unsafe fn get_hdrs_offset(kernelfd: libc::c_int, filename: *const libc::c_char) -> libc::off_t {
    let mut buffer = [0 as libc::c_char; 1024];
    if libc::lseek(kernelfd, 0, libc::SEEK_SET) < 0 { die(b"lseek\0".as_ptr() as *const libc::c_char); }
    if libc::read(kernelfd, buffer.as_mut_ptr() as *mut libc::c_void, BUFSIZE as usize) != BUFSIZE as isize { die(filename); }
    if buffer[40] == b'H' as libc::c_char && buffer[41] == b'd' as libc::c_char && buffer[42] == b'r' as libc::c_char && buffer[43] == b'S' as libc::c_char { return 40; }
    let mut offset = ((ld2(buffer.as_mut_ptr().add(AOUT_TEXT_OFFSET as usize + 2)) as libc::off_t) << 2) - LOOKBACK + AOUT_TEXT_OFFSET;
    if offset < 0 { libc::set_errno(libc::errno::consts::EINVAL); die(b"Calculated a negative offset, probably elftoaout generated an invalid image. Did you use a recent elftoaout ?\0".as_ptr() as *const libc::c_char); }
    if libc::lseek(kernelfd, offset, libc::SEEK_SET) < 0 { die(b"lseek\0".as_ptr() as *const libc::c_char); }
    if libc::read(kernelfd, buffer.as_mut_ptr() as *mut libc::c_void, BUFSIZE as usize) != BUFSIZE as isize { die(filename); }
    for i in (0..LOOKBACK as usize).step_by(4) { if buffer[i] == b'H' as libc::c_char && buffer[i+1] == b'd' as libc::c_char && buffer[i+2] == b'r' as libc::c_char && buffer[i+3] == b'S' as libc::c_char { return offset + i as libc::off_t; } }
    libc::fprintf(libc::stderr, b"Couldn't find headers signature in %s\n\0".as_ptr() as *const libc::c_char, filename); libc::exit(1)
}

pub unsafe fn main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int {
    let aout_magic = [1u8, 3, 1, 7]; let mut buffer = [0 as libc::c_char; 1024];
    if argc != 5 { usage(); }
    if libc::strcmp(*argv.add(1), b"64\0".as_ptr() as *const libc::c_char) == 0 { IS64BIT = 1; }
    let mut s = core::mem::MaybeUninit::<libc::stat>::uninit(); if libc::stat(*argv.add(4), s.as_mut_ptr()) < 0 { die(*argv.add(4)); } let s = s.assume_init();
    let (mut start, mut end) = (0u32, 0u32); if get_start_end(*argv.add(3), &mut start, &mut end) == 0 { libc::fprintf(libc::stderr, b"Could not determine start and end from %s\n\0".as_ptr() as *const libc::c_char, *argv.add(3)); libc::exit(1); }
    let image = libc::open(*argv.add(2), libc::O_RDWR); if image < 0 { die(*argv.add(2)); }
    if libc::read(image, buffer.as_mut_ptr() as *mut libc::c_void, 512) != 512 { die(*argv.add(2)); }
    if libc::memcmp(buffer.as_ptr() as *const libc::c_void, aout_magic.as_ptr() as *const libc::c_void, 4) != 0 { libc::fprintf(libc::stderr, b"Not a.out. Don't blame me.\n\0".as_ptr() as *const libc::c_char); libc::exit(1); }
    let offset = get_hdrs_offset(image, *argv.add(2)) + 10; if libc::lseek(image, offset, 0) < 0 { die(b"lseek\0".as_ptr() as *const libc::c_char); }
    st4(buffer.as_mut_ptr(), 0); st4(buffer.as_mut_ptr().add(4), 0x01000000); st4(buffer.as_mut_ptr().add(8), align((end + 32) as libc::c_int) as libc::c_uint); st4(buffer.as_mut_ptr().add(12), s.st_size as libc::c_uint);
    if libc::write(image, buffer.as_ptr().add(2) as *const libc::c_void, 14) != 14 { die(*argv.add(2)); }
    if IS64BIT != 0 { if libc::lseek(image, 4, 0) < 0 { die(b"lseek\0".as_ptr() as *const libc::c_char); } st4(buffer.as_mut_ptr(), (align((end + 32 + 8191) as libc::c_int) as libc::c_uint).wrapping_sub(start & !0x3fffff).wrapping_add(s.st_size as libc::c_uint)); st4(buffer.as_mut_ptr().add(4), 0); st4(buffer.as_mut_ptr().add(8), 0); if libc::write(image, buffer.as_ptr() as *const libc::c_void, 12) != 12 { die(*argv.add(2)); } }
    if libc::lseek(image, AOUT_TEXT_OFFSET - start as libc::off_t + align((end + 32) as libc::c_int) as libc::off_t, 0) < 0 { die(b"lseek\0".as_ptr() as *const libc::c_char); }
    let tail = libc::open(*argv.add(4), libc::O_RDONLY); if tail < 0 { die(*argv.add(4)); }
    let mut i: isize; while { i = libc::read(tail, buffer.as_mut_ptr() as *mut libc::c_void, 1024); i > 0 } { if libc::write(image, buffer.as_ptr() as *const libc::c_void, i as usize) != i { die(*argv.add(2)); } }
    if libc::close(image) < 0 { die(b"close\0".as_ptr() as *const libc::c_char); } if libc::close(tail) < 0 { die(b"close\0".as_ptr() as *const libc::c_char); } 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
