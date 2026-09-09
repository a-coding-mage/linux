// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * hack-coff.c - hack the header of an xcoff file to fill in
 * a few fields needed by the Open Firmware xcoff loader on
 * Power Macs but not initialized by objcopy.
 *
 * Copyright (C) Paul Mackerras 1997.
 */

use std::ffi::CString;
use std::mem::{size_of, zeroed};
use std::os::raw::{c_char, c_int, c_long, c_void};
use std::process::exit;

// Supplied by rs6000.h.
const U802TOCMAGIC: u16 = 0x01df;
const U802WRMAGIC: u16 = 0x01ef;
const U802ROMAGIC: u16 = 0x01f7;

const AOUT_MAGIC: u16 = 0x010b;

#[repr(C)]
struct external_filehdr {
    f_magic: [u8; 2],
    f_nscns: [u8; 2],
    _rest: [u8; 12],
    f_opthdr: [u8; 2],
    _tail: [u8; 2],
}

#[repr(C)]
struct aouthdr {
    magic: [u8; 2],
    _vstamp: [u8; 2],
    _tsize: [u8; 4],
    _dsize: [u8; 4],
    _bsize: [u8; 4],
    _entry: [u8; 4],
    _text_start: [u8; 4],
    _data_start: [u8; 4],
    o_toc: [u8; 4],
    o_snentry: [u8; 2],
    o_sntext: [u8; 2],
    o_sndata: [u8; 2],
    o_sntoc: [u8; 2],
    o_snloader: [u8; 2],
    o_snbss: [u8; 2],
    _pad: [u8; 2],
}

#[repr(C)]
struct external_scnhdr {
    s_name: [c_char; 8],
    _rest: [u8; 32],
}

unsafe extern "C" {
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn lseek(fd: c_int, offset: c_long, whence: c_int) -> c_long;
    fn close(fd: c_int) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    static mut stderr: *mut c_void;
}

unsafe fn get_16be(x: *const u8) -> u16 {
    ((*x as u16) << 8) + (*x.add(1) as u16)
}

unsafe fn put_16be(x: *mut u8, v: u16) {
    *x = (v >> 8) as u8;
    *x.add(1) = (v & 0xff) as u8;
}

fn main() {
    unsafe {
        let args: Vec<CString> = std::env::args().map(|s| CString::new(s).unwrap()).collect();
        let ac = args.len() as c_int;
        let av: Vec<*const c_char> = args.iter().map(|s| s.as_ptr()).collect();
        let mut fd: c_int;
        let mut i: c_int;
        let mut nsect: c_int;
        let mut aoutsz: usize;
        let mut fhdr: external_filehdr = zeroed();
        let mut aout: aouthdr = zeroed();
        let mut shdr: external_scnhdr = zeroed();
        if ac != 2 {
            fprintf(stderr, b"Usage: hack-coff coff-file\0".as_ptr() as *const c_char);
            exit(1);
        }
        fd = open(av[1], 2);
        if fd == -1 {
            perror(av[2]);
            exit(1);
        }
        if read(fd, &mut fhdr as *mut _ as *mut c_void, size_of::<external_filehdr>()) != size_of::<external_filehdr>() as isize { goto_readerr(); }
        i = get_16be(fhdr.f_magic.as_ptr()) as c_int;
        if i != U802TOCMAGIC as c_int && i != U802WRMAGIC as c_int && i != U802ROMAGIC as c_int {
            fprintf(stderr, b"%s: not an xcoff file\n\0".as_ptr() as *const c_char, av[1]); exit(1);
        }
        aoutsz = get_16be(fhdr.f_opthdr.as_ptr()) as usize;
        if read(fd, &mut aout as *mut _ as *mut c_void, aoutsz) != aoutsz as isize { goto_readerr(); }
        nsect = get_16be(fhdr.f_nscns.as_ptr()) as c_int;
        i = 0;
        while i < nsect {
            if read(fd, &mut shdr as *mut _ as *mut c_void, size_of::<external_scnhdr>()) != size_of::<external_scnhdr>() as isize { goto_readerr(); }
            let name = &shdr.s_name;
            if name == b".text\0\0\0" { put_16be(aout.o_snentry.as_mut_ptr(), (i + 1) as u16); put_16be(aout.o_sntext.as_mut_ptr(), (i + 1) as u16); }
            else if name == b".data\0\0\0" { put_16be(aout.o_sndata.as_mut_ptr(), (i + 1) as u16); }
            else if name == b".bss\0\0\0\0" { put_16be(aout.o_snbss.as_mut_ptr(), (i + 1) as u16); }
            i += 1;
        }
        put_16be(aout.magic.as_mut_ptr(), AOUT_MAGIC);
        if lseek(fd, size_of::<external_filehdr>() as c_long, 0) == -1 || write(fd, &aout as *const _ as *const c_void, aoutsz) != aoutsz as isize {
            fprintf(stderr, b"%s: write error\n\0".as_ptr() as *const c_char, av[1]); exit(1);
        }
        close(fd); exit(0);
    }
}

#[inline(never)]
fn goto_readerr() -> ! {
    unsafe { fprintf(stderr, b"%s: read error or file too short\n\0".as_ptr() as *const c_char, std::env::args().nth(1).unwrap_or_default().as_ptr() as *const c_char); }
    exit(1)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
