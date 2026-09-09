// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2009 Thomas Chou <thomas@wytron.com.tw>
 *
 * This is a collection of several routines from gzip-1.0.3
 * adapted for Linux.
 *
 * malloc by Hannu Savolainen 1993 and Matthias Urlichs 1994
 *
 * Adapted for SH by Stuart Menefy, Aug 1999
 *
 * Modified to use standard LinuxSH BIOS by Greg Banks 7Jul2000
 *
 * Based on arch/sh/boot/compressed/misc.c
 */

// Dependency supplied by linux/string.h.

pub type uch = u8;
pub type ush = u16;
pub type ulg = usize;

pub const WSIZE: usize = 0x8000;

static mut inbuf: *mut uch = core::ptr::null_mut();
static mut window: [uch; WSIZE] = [0; WSIZE];
static mut insize: u32 = 0;
static mut inptr: u32 = 0;
static mut outcnt: u32 = 0;

pub const ASCII_FLAG: u32 = 0x01;
pub const CONTINUATION: u32 = 0x02;
pub const EXTRA_FIELD: u32 = 0x04;
pub const ORIG_NAME: u32 = 0x08;
pub const COMMENT: u32 = 0x10;
pub const ENCRYPTED: u32 = 0x20;
pub const RESERVED: u32 = 0xC0;

extern "C" {
    static mut input_data: [uch; 0];
    static mut input_len: i32;
    static mut _end: i32;
    static mut CONFIG_NIOS2_MEM_BASE: usize;
    static mut CONFIG_NIOS2_KERNEL_REGION_BASE: usize;
    static mut crc: ulg;
    static crc_32_tab: [ulg; 256];
    fn gunzip();
    fn makecrc();
}

static mut bytes_out: i64 = 0;
static mut output_data: *mut uch = core::ptr::null_mut();
static mut output_ptr: usize = 0;

// Declaration and implementation supplied by console.c.
extern "C" {
    fn console_init();
}

extern "C" {
    fn puts(s: *const core::ffi::c_char) -> i32;
}

static mut free_mem_ptr: usize = 0;
static mut free_mem_end_ptr: usize = 0;
pub const HEAP_SIZE: usize = 0x10000;

// The implementation included from ../../../../lib/inflate.c is supplied externally.

#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void {
    let ss = s as *mut u8;
    for i in 0..n {
        *ss.add(i) = c as u8;
    }
    s
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void {
    let d = dest as *mut u8;
    let s = src as *const u8;
    for i in 0..n {
        *d.add(i) = *s.add(i);
    }
    dest
}

/*
 * Fill the input buffer. This is called only when the buffer is empty
 * and at least one byte is really needed.
 */
unsafe fn fill_inbuf() -> i32 {
    if insize != 0 {
        error(c"ran out of input data".as_ptr() as *mut _);
    }

    inbuf = input_data.as_mut_ptr();
    insize = input_len as u32;
    inptr = 1;
    *inbuf as i32
}

/*
 * Write the output window window[0..outcnt-1] and update crc and bytes_out.
 * (Used for the decompressed data only.)
 */
unsafe fn flush_window() {
    let mut c = crc;
    let mut input = window.as_mut_ptr();
    let mut output = output_data.add(output_ptr);
    let n = outcnt as usize;

    for _ in 0..n {
        let ch = *input;
        *output = ch;
        output = output.add(1);
        input = input.add(1);
        c = crc_32_tab[((c as i32 ^ ch as i32) & 0xff) as usize] ^ (c >> 8);
    }
    crc = c;
    bytes_out += outcnt as i64;
    output_ptr += outcnt as usize;
    outcnt = 0;
}

unsafe fn error(x: *mut core::ffi::c_char) -> ! {
    puts(c"\nERROR\n".as_ptr());
    puts(x);
    puts(c"\n\n -- System halted".as_ptr());
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn decompress_kernel() {
    output_data = (CONFIG_NIOS2_MEM_BASE | CONFIG_NIOS2_KERNEL_REGION_BASE) as *mut uch;
    output_ptr = 0;
    free_mem_ptr = (&raw mut _end) as *mut i32 as usize;
    free_mem_end_ptr = free_mem_ptr + HEAP_SIZE;

    console_init();
    makecrc();
    puts(c"Uncompressing Linux... ".as_ptr());
    gunzip();
    puts(c"Ok, booting the kernel.\n".as_ptr());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
