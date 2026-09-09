// SPDX-License-Identifier: GPL-2.0
/*
 * misc.c
 *
 * This is a collection of several routines from gzip-1.0.3
 * adapted for Linux.
 *
 * malloc by Hannu Savolainen 1993 and Matthias Urlichs 1994
 *
 * Modified for ARM Linux by Russell King
 *
 * Nicolas Pitre <nico@visuaide.com>  1999/04/14 :
 *  For this code to run directly from Flash, all constant variables must
 *  be marked with 'const' and all other variables initialized at run-time
 *  only.  This way all non constant variables will end up in the bss segment,
 *  which should point to addresses in RAM and cleared to 0 on start.
 *
 *  This allows for a much quicker boot time.
 *
 * Modified for Alpha, from the ARM version, by Jay Estabrook 2003.
 */

use core::ffi::{c_char, c_int, c_void};

// The Linux kernel headers and ../../../lib/inflate.c provide the remaining
// symbols and definitions used by this translation.

type Uch = u8;
type Ush = u16;
type Ulg = c_ulong;
type c_ulong = usize;

const WSIZE: usize = 0x8000;

static mut inbuf: *mut Uch = core::ptr::null_mut();
static mut window: *mut Uch = core::ptr::null_mut();
static mut insize: c_uint = 0;
static mut inptr: c_uint = 0;
static mut outcnt: c_uint = 0;

const ASCII_FLAG: c_int = 0x01;
const CONTINUATION: c_int = 0x02;
const EXTRA_FIELD: c_int = 0x04;
const ORIG_NAME: c_int = 0x08;
const COMMENT: c_int = 0x10;
const ENCRYPTED: c_int = 0x20;
const RESERVED: c_int = 0xC0;

extern "C" {
    fn srm_printk(format: *const c_char, ...) -> c_int;
    fn malloc(size: usize) -> *mut Uch;
    fn makecrc();
    fn gunzip();
}

type c_uint = u32;

static mut input_data: *mut c_char = core::ptr::null_mut();
static mut input_data_size: c_int = 0;
static mut output_data: *mut Uch = core::ptr::null_mut();
static mut output_ptr: Ulg = 0;
static mut bytes_out: Ulg = 0;

extern "C" {
    static mut end: c_int;
}

static mut free_mem_ptr: Ulg = 0;
static mut free_mem_end_ptr: Ulg = 0;

const HEAP_SIZE: usize = 0x3000;

// The original source includes ../../../lib/inflate.c here; its declarations
// and globals are supplied by the corresponding Rust translation.

pub unsafe extern "C" fn fill_inbuf() -> c_int {
    if insize != 0 {
        error(b"ran out of input data\0".as_ptr() as *mut c_char);
    }

    inbuf = input_data as *mut Uch;
    insize = input_data_size as c_uint;

    inptr = 1;
    *inbuf as c_int
}

pub unsafe extern "C" fn flush_window() {
    let mut c: Ulg = crc;
    let mut n: c_uint;
    let mut input: *mut Uch;
    let mut output: *mut Uch;
    let mut ch: Uch;

    input = window;
    output = output_data.add(output_ptr as usize);
    n = 0;
    while n < outcnt {
        ch = *input;
        *output = ch;
        output = output.add(1);
        input = input.add(1);
        c = crc_32_tab[((c as c_int ^ ch as c_int) & 0xff) as usize] ^ (c >> 8);
        n += 1;
    }
    crc = c;
    bytes_out += outcnt as Ulg;
    output_ptr += outcnt as Ulg;
    outcnt = 0;
    // puts(".");
}

unsafe fn error(x: *mut c_char) -> ! {
    srm_printk(b"\n\n\0".as_ptr() as *const c_char);
    srm_printk(x as *const c_char);
    srm_printk(b"\n\n -- System halted\0".as_ptr() as *const c_char);

    loop {}
}

pub unsafe extern "C" fn decompress_kernel(
    output_start: *mut c_void,
    input_start: *mut c_void,
    _ksize: usize,
    kzsize: usize,
) -> c_uint {
    output_data = output_start as *mut Uch;
    input_data = input_start as *mut c_char;
    input_data_size = kzsize as c_int;

    // FIXME FIXME FIXME
    free_mem_ptr = output_start as Ulg + _ksize;
    free_mem_end_ptr = output_start as Ulg + _ksize + 0x200000;
    // FIXME FIXME FIXME

    // put in temp area to reduce initial footprint
    window = malloc(WSIZE);

    makecrc();
    // puts("Uncompressing Linux...");
    gunzip();
    // puts(" done, booting the kernel.\n");
    output_ptr as c_uint
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
