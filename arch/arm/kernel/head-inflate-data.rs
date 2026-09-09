// SPDX-License-Identifier: GPL-2.0-only
/*
 * XIP kernel .data segment decompressor
 *
 * Created by: Nicolas Pitre, August 2017
 * Copyright: (C) 2017 Linaro Limited
 */

use core::ffi::{c_char, c_int, c_void};

/* Types and symbols supplied by the corresponding kernel/zlib dependencies. */
#[repr(C)]
struct z_stream_s {
    next_in: *mut u8,
    avail_in: u32,
    next_out: *mut u8,
    avail_out: u32,
    workspace: *mut c_void,
}

#[repr(C)]
struct inflate_state {
    wsize: u32,
    window: *mut u8,
}

unsafe extern "C" {
    static mut __data_loc: *mut c_char;
    static mut _edata_loc: *mut c_char;
    static mut _sdata: *mut u8;

    fn zlib_inflateInit2(strm: *mut z_stream_s, window_bits: c_int) -> c_int;
    fn zlib_inflate(strm: *mut z_stream_s, flush: c_int) -> c_int;
}

const MAX_WBITS: c_int = 15;
const Z_FINISH: c_int = 4;
const Z_OK: c_int = 0;
const Z_STREAM_END: c_int = 1;

/*
 * This code is called very early during the boot process to decompress
 * the .data segment stored compressed in ROM. Therefore none of the global
 * variables are valid yet, hence no kernel services such as memory
 * allocation is available. Everything must be allocated on the stack and
 * we must avoid any global data access. We use a temporary stack located
 * in the .bss area. The linker script makes sure the .bss is big enough
 * to hold our stack frame plus some room for called functions.
 *
 * We mimic the code in lib/decompress_inflate.c to use the smallest work
 * area possible. And because everything is statically allocated on the
 * stack then there is no need to clean up before returning.
 */

pub unsafe fn __inflate_kernel_data() -> c_int {
    let mut stream: z_stream_s = core::mem::zeroed();
    let strm: *mut z_stream_s = &mut stream;
    let mut state: inflate_state = core::mem::zeroed();
    let mut input: *mut u8 = __data_loc.cast();
    let rc: c_int;

    /* Check and skip gzip header (assume no filename) */
    if *input.add(0) != 0x1f
        || *input.add(1) != 0x8b
        || *input.add(2) != 0x08
        || (*input.add(3) & !3) != 0
    {
        return -1;
    }
    input = input.add(10);

    (*strm).workspace = (&mut state as *mut inflate_state).cast();
    (*strm).next_in = input;
    (*strm).avail_in = (_edata_loc as usize - __data_loc as usize) as u32; /* upper bound */
    (*strm).next_out = _sdata;
    (*strm).avail_out = (_edata_loc as usize - __data_loc as usize) as u32;
    zlib_inflateInit2(strm, -MAX_WBITS);
    (*strm).workspace.cast::<inflate_state>().as_mut().unwrap().wsize = 0;
    (*strm).workspace.cast::<inflate_state>().as_mut().unwrap().window = core::ptr::null_mut();
    rc = zlib_inflate(strm, Z_FINISH);
    if rc == Z_OK || rc == Z_STREAM_END {
        return (*strm).avail_out as c_int; /* should be 0 */
    }
    rc
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
