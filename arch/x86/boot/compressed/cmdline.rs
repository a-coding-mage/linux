// SPDX-License-Identifier: GPL-2.0

// Dependency supplied by misc.h.
// Dependency supplied by asm/bootparam.h.
// The declarations from ../cmdline.c are shared with the surrounding source.

static mut FS: usize = 0;

#[inline]
unsafe fn set_fs(seg: usize) {
    FS = seg << 4; // shift it back
}

type AddrT = usize;

#[inline]
unsafe fn rdfs8(addr: AddrT) -> i8 {
    *((FS.wrapping_add(addr)) as *const i8)
}

pub unsafe fn get_cmd_line_ptr() -> usize {
    let mut cmd_line_ptr = boot_params_ptr.hdr.cmd_line_ptr;

    cmd_line_ptr |= (boot_params_ptr.ext_cmd_line_ptr as u64) << 32;

    cmd_line_ptr
}

pub unsafe fn cmdline_find_option(
    option: *const i8,
    buffer: *mut i8,
    bufsize: i32,
) -> i32 {
    __cmdline_find_option(get_cmd_line_ptr(), option, buffer, bufsize)
}

pub unsafe fn cmdline_find_option_bool(option: *const i8) -> i32 {
    __cmdline_find_option_bool(get_cmd_line_ptr(), option)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
