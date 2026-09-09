// SPDX-License-Identifier: GPL-2.0
/*
 * Serial port routines for use during early boot reporting. This code is
 * included from both the compressed kernel and the regular kernel.
 */

use core::ffi::{c_char, c_void};

// Dependency declarations supplied by boot.h and other translation units.
unsafe extern "C" {
    fn outb(value: u8, port: i32);
    fn inb(port: i32) -> u8;
    fn cmdline_find_option(option: *const c_char, buffer: *mut c_char, len: usize) -> i32;
    fn simple_strtoull(value: *const c_char, endp: *mut *mut c_char, base: u32) -> u64;
    fn strncmp(a: *const c_char, b: *const c_char, len: usize) -> i32;
    static mut early_serial_base: i32;
}

const DEFAULT_SERIAL_PORT: i32 = 0x3f8;
const DLAB: u8 = 0x80;
const TXR: i32 = 0;
const RXR: i32 = 0;
const IER: i32 = 1;
const IIR: i32 = 2;
const FCR: i32 = 2;
const LCR: i32 = 3;
const MCR: i32 = 4;
const LSR: i32 = 5;
const MSR: i32 = 6;
const DLL: i32 = 0;
const DLH: i32 = 1;
const DEFAULT_BAUD: i32 = 9600;

unsafe fn early_serial_init(port: i32, baud: i32) {
    let mut c: u8;
    let divisor: u32;

    outb(0x3, port + LCR); // 8n1
    outb(0, port + IER); // no interrupt
    outb(0, port + FCR); // no fifo
    outb(0x3, port + MCR); // DTR + RTS

    divisor = (115200 / baud) as u32;
    c = inb(port + LCR);
    outb(c | DLAB, port + LCR);
    outb((divisor & 0xff) as u8, port + DLL);
    outb(((divisor >> 8) & 0xff) as u8, port + DLH);
    outb(c & !DLAB, port + LCR);

    early_serial_base = port;
}

unsafe fn parse_earlyprintk() {
    let mut baud = DEFAULT_BAUD;
    let mut arg = [0 as c_char; 32];
    let mut pos: usize = 0;
    let mut port = 0;
    let earlyprintk = b"earlyprintk\0";

    if cmdline_find_option(earlyprintk.as_ptr() as *const c_char, arg.as_mut_ptr(), arg.len()) > 0 {
        let mut e: *mut c_char;

        if strncmp(arg.as_ptr(), b"serial\0".as_ptr() as *const c_char, 6) == 0 {
            port = DEFAULT_SERIAL_PORT;
            pos += 6;
        }

        if arg[pos] == b',' as c_char { pos += 1; }

        /* make sure we have "serial,0x3f8,115200", "serial,ttyS0,115200", "ttyS0,115200" */
        if pos == 7 && strncmp(arg.as_ptr().add(pos), b"0x\0".as_ptr() as *const c_char, 2) == 0 {
            port = simple_strtoull(arg.as_ptr().add(pos), &mut e, 16) as i32;
            if port == 0 || arg.as_ptr().add(pos) == e { port = DEFAULT_SERIAL_PORT; }
            else { pos = e.offset_from(arg.as_ptr()) as usize; }
        } else if strncmp(arg.as_ptr().add(pos), b"ttyS\0".as_ptr() as *const c_char, 4) == 0 {
            let bases = [0x3f8, 0x2f8];
            let mut idx = 0;
            pos += 4;
            if arg[pos] == b'1' as c_char { idx = 1; }
            pos += 1;
            port = bases[idx];
        }

        if arg[pos] == b',' as c_char { pos += 1; }
        baud = simple_strtoull(arg.as_ptr().add(pos), &mut e, 0) as i32;
        if baud == 0 || arg.as_ptr().add(pos) == e { baud = DEFAULT_BAUD; }
    }

    if port != 0 { early_serial_init(port, baud); }
}

const BASE_BAUD: u32 = 1843200 / 16;
unsafe fn probe_baud(port: i32) -> u32 {
    let lcr = inb(port + LCR);
    outb(lcr | DLAB, port + LCR);
    let dll = inb(port + DLL);
    let dlh = inb(port + DLH);
    outb(lcr, port + LCR);
    let quot = ((dlh as u32) << 8) | dll as u32;
    BASE_BAUD / quot
}

unsafe fn parse_console_uart8250() {
    let mut optstr = [0 as c_char; 64];
    let mut options: *mut c_char;
    let baud: i32;
    let mut port = 0;
    let console = b"console\0";

    if cmdline_find_option(console.as_ptr() as *const c_char, optstr.as_mut_ptr(), optstr.len()) <= 0 { return; }
    options = optstr.as_mut_ptr();
    if strncmp(options, b"uart8250,io,\0".as_ptr() as *const c_char, 12) == 0 {
        port = simple_strtoull(options.add(12), &mut options, 0) as i32;
    } else if strncmp(options, b"uart,io,\0".as_ptr() as *const c_char, 8) == 0 {
        port = simple_strtoull(options.add(8), &mut options, 0) as i32;
    } else { return; }
    if !options.is_null() && *options == b',' as c_char {
        baud = simple_strtoull(options.add(1), core::ptr::null_mut(), 0) as i32;
        let baud = if baud == 0 { DEFAULT_BAUD } else { baud };
        if port != 0 { early_serial_init(port, baud); }
    } else {
        baud = probe_baud(port) as i32;
        if port != 0 { early_serial_init(port, baud); }
    }
}

pub unsafe fn console_init() {
    parse_earlyprintk();
    if early_serial_base == 0 { parse_console_uart8250(); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
