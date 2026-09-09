/* SPDX-License-Identifier: GPL-2.0 */
/*
 *	include/linux/vt_buffer.h -- Access to VT screen buffer
 *
 *	(c) 1998 Martin Mares <mj@ucw.cz>
 *
 *	This is a set of macros and functions which are used in the
 *	console driver and related code to access the screen buffer.
 *	In most cases the console works with simple in-memory buffer,
 *	but when handling hardware text mode consoles, we store the
 *	foreground console directly in video memory.
 */

/* <linux/string.h> and, when CONFIG_VGA_CONSOLE is enabled, <asm/vga.h>. */

/* The following declarations are supplied by the corresponding dependencies. */
unsafe extern "C" {
    fn memset16(s: *mut u16, c: u16, count: usize);
    fn memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, count: usize)
        -> *mut core::ffi::c_void;
    fn memmove(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, count: usize)
        -> *mut core::ffi::c_void;
}

/* VT_BUF_HAVE_RW: use the platform-provided read/write operations when defined. */
#[macro_export]
macro_rules! scr_writew {
    ($val:expr, $addr:expr) => {{
        unsafe { *($addr) = $val };
    }};
}

#[macro_export]
macro_rules! scr_readw {
    ($addr:expr) => {{
        unsafe { *($addr) }
    }};
}

/* VT_BUF_HAVE_MEMSETW: use the platform-provided implementation when defined. */
#[inline]
pub unsafe fn scr_memsetw(s: *mut u16, c: u16, count: u32) {
    unsafe {
        memset16(s, c, (count / 2) as usize);
    }
}

/* VT_BUF_HAVE_MEMCPYW: use the platform-provided implementation when defined. */
#[inline]
pub unsafe fn scr_memcpyw(d: *mut u16, s: *const u16, count: u32) {
    unsafe {
        memcpy(
            d as *mut core::ffi::c_void,
            s as *const core::ffi::c_void,
            count as usize,
        );
    }
}

/* VT_BUF_HAVE_MEMMOVEW: use the platform-provided implementation when defined. */
#[inline]
pub unsafe fn scr_memmovew(d: *mut u16, s: *const u16, count: u32) {
    unsafe {
        memmove(
            d as *mut core::ffi::c_void,
            s as *const core::ffi::c_void,
            count as usize,
        );
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
