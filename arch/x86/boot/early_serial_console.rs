// SPDX-License-Identifier: GPL-2.0
/*
 * Serial port routines for use during early boot reporting. This code is
 * included from both the compressed kernel and the regular kernel.
 */

use super::EARLY_SERIAL_BASE;
use crate::bindings as b;
use core::ffi::{c_char, c_int, c_uint};
use core::mem::MaybeUninit;
use core::ptr::null_mut;

const DEFAULT_SERIAL_PORT: c_int = 0x3f8;
const DLAB: u8 = 0x80;
const IER: c_int = 1;
const FCR: c_int = 2;
const LCR: c_int = 3;
const MCR: c_int = 4;
const DLL: c_int = 0;
const DLH: c_int = 1;
const DEFAULT_BAUD: c_int = 9600;

// boot/io.h dispatches through the runtime-installed callbacks, including the
// TDX replacements. Its u16 port argument truncates the original C int.
#[inline]
unsafe fn inb(port: c_int) -> u8 {
    unsafe { (b::pio_ops.f_inb.unwrap_unchecked())(port as u16) }
}

#[inline]
unsafe fn outb(value: u8, port: c_int) {
    unsafe { (b::pio_ops.f_outb.unwrap_unchecked())(value, port as u16) }
}

unsafe fn early_serial_init(port: c_int, baud: c_int) {
    unsafe {
        outb(0x3, port.wrapping_add(LCR)); // 8n1
        outb(0, port.wrapping_add(IER)); // no interrupt
        outb(0, port.wrapping_add(FCR)); // no fifo
        outb(0x3, port.wrapping_add(MCR)); // DTR + RTS

        // The C expression divides signed ints before assigning unsigned.
        let divisor = (115200 / baud) as c_uint;
        let c = inb(port.wrapping_add(LCR));
        outb(c | DLAB, port.wrapping_add(LCR));
        outb((divisor & 0xff) as u8, port.wrapping_add(DLL));
        outb(((divisor >> 8) & 0xff) as u8, port.wrapping_add(DLH));
        outb(c & !DLAB, port.wrapping_add(LCR));

        EARLY_SERIAL_BASE = port;
    }
}

unsafe fn parse_earlyprintk() {
    unsafe {
        let mut baud = DEFAULT_BAUD;
        let mut arg_storage = MaybeUninit::<[c_char; 32]>::uninit();
        let arg = arg_storage.as_mut_ptr().cast::<c_char>();
        let mut pos: c_int = 0;
        let mut port: c_int = 0;

        if b::cmdline_find_option(c"earlyprintk".as_ptr(), arg, 32) > 0 {
            let mut e = null_mut();

            if b::strncmp(arg, c"serial".as_ptr(), 6) == 0 {
                port = DEFAULT_SERIAL_PORT;
                pos += 6;
            }

            if *arg.offset(pos as isize) == b',' as c_char {
                pos += 1;
            }

            /*
             * Supported forms include "serial,0x3f8,115200",
             * "serial,ttyS0,115200", and "ttyS0,115200".
             */
            if pos == 7 && b::strncmp(arg.offset(pos as isize), c"0x".as_ptr(), 2) == 0 {
                port = b::simple_strtoull(arg.offset(pos as isize), &mut e, 16) as c_int;
                if port == 0 || arg.offset(pos as isize) == e {
                    port = DEFAULT_SERIAL_PORT;
                } else {
                    pos = e.offset_from(arg) as c_int;
                }
            } else if b::strncmp(arg.offset(pos as isize), c"ttyS".as_ptr(), 4) == 0 {
                let bases = [0x3f8, 0x2f8];
                let mut idx = 0;
                pos += 4;
                if *arg.offset(pos as isize) == b'1' as c_char {
                    idx = 1;
                }
                pos += 1;
                port = bases[idx];
            }

            if *arg.offset(pos as isize) == b',' as c_char {
                pos += 1;
            }

            baud = b::simple_strtoull(arg.offset(pos as isize), &mut e, 0) as c_int;
            if baud == 0 || arg.offset(pos as isize) == e {
                baud = DEFAULT_BAUD;
            }
        }

        if port != 0 {
            early_serial_init(port, baud);
        }
    }
}

const BASE_BAUD: c_uint = 1843200 / 16;

unsafe fn probe_baud(port: c_int) -> c_uint {
    unsafe {
        let lcr = inb(port.wrapping_add(LCR));
        outb(lcr | DLAB, port.wrapping_add(LCR));
        let dll = inb(port.wrapping_add(DLL));
        let dlh = inb(port.wrapping_add(DLH));
        outb(lcr, port.wrapping_add(LCR));
        let quot = ((dlh as c_uint) << 8) | dll as c_uint;

        BASE_BAUD / quot
    }
}

unsafe fn parse_console_uart8250() {
    unsafe {
        let mut optstr_storage = MaybeUninit::<[c_char; 64]>::uninit();
        let optstr = optstr_storage.as_mut_ptr().cast::<c_char>();

        // The command-line parser returns the last console= occurrence.
        if b::cmdline_find_option(c"console".as_ptr(), optstr, 64) <= 0 {
            return;
        }

        let mut options = optstr;
        let port = if b::strncmp(options, c"uart8250,io,".as_ptr(), 12) == 0 {
            b::simple_strtoull(options.add(12), &mut options, 0) as c_int
        } else if b::strncmp(options, c"uart,io,".as_ptr(), 8) == 0 {
            b::simple_strtoull(options.add(8), &mut options, 0) as c_int
        } else {
            return;
        };

        let baud = if !options.is_null() && *options == b',' as c_char {
            let baud = b::simple_strtoull(options.add(1), null_mut(), 0) as c_int;
            if baud == 0 {
                DEFAULT_BAUD
            } else {
                baud
            }
        } else {
            // The C code probes before checking port, even when port is zero.
            probe_baud(port) as c_int
        };

        if port != 0 {
            early_serial_init(port, baud);
        }
    }
}

/// Initialize early serial output from the boot command line.
///
/// # Safety
/// The native boot command line is readable and the port I/O callbacks have
/// been installed. UART accesses and parser inputs meet the original C
/// routine's contract, including a nonzero divisor when probing the baud rate.
#[no_mangle]
pub(crate) unsafe extern "C" fn console_init() {
    unsafe {
        parse_earlyprintk();
        if EARLY_SERIAL_BASE == 0 {
            parse_console_uart8250();
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
