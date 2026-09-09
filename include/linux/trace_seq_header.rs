/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from linux/trace_seq.h.  The included kernel types and helpers
 * are supplied by other translated dependencies. */

pub const TRACE_SEQ_SIZE: usize = 8192;
pub const TRACE_SEQ_BUFFER_SIZE: usize =
    TRACE_SEQ_SIZE - (core::mem::size_of::<seq_buf>() + core::mem::size_of::<usize>() + core::mem::size_of::<i32>());

#[repr(C)]
pub struct trace_seq {
    pub seq: seq_buf,
    pub readpos: usize,
    pub full: i32,
    pub buffer: [core::ffi::c_char; TRACE_SEQ_BUFFER_SIZE],
}

#[inline]
pub unsafe fn trace_seq_init(s: *mut trace_seq) {
    seq_buf_init(
        &mut (*s).seq,
        (*s).buffer.as_mut_ptr(),
        TRACE_SEQ_BUFFER_SIZE,
    );
    (*s).full = 0;
    (*s).readpos = 0;
}

#[inline]
pub unsafe fn trace_seq_used(s: *mut trace_seq) -> i32 {
    seq_buf_used(&mut (*s).seq)
}

#[inline]
pub unsafe fn trace_seq_buffer_ptr(s: *mut trace_seq) -> *mut core::ffi::c_char {
    (*s).buffer.as_mut_ptr().add(seq_buf_used(&mut (*s).seq) as usize)
}

#[inline]
pub unsafe fn trace_seq_has_overflowed(s: *mut trace_seq) -> bool {
    (*s).full != 0 || seq_buf_has_overflowed(&mut (*s).seq)
}

#[inline]
pub unsafe fn trace_seq_pop(s: *mut trace_seq) -> i32 {
    seq_buf_pop(&mut (*s).seq)
}

/* Currently only defined when tracing is enabled. */
#[cfg(feature = "CONFIG_TRACING")]
extern "C" {
    pub fn trace_seq_printf(s: *mut trace_seq, fmt: *const core::ffi::c_char, ...);
    pub fn trace_seq_vprintf(s: *mut trace_seq, fmt: *const core::ffi::c_char, args: va_list);
    pub fn trace_seq_bprintf(s: *mut trace_seq, fmt: *const core::ffi::c_char, binary: *const u32);
    pub fn trace_print_seq(m: *mut seq_file, s: *mut trace_seq) -> i32;
    pub fn trace_seq_to_user(s: *mut trace_seq, ubuf: *mut core::ffi::c_char, cnt: i32) -> i32;
    pub fn trace_seq_puts(s: *mut trace_seq, str_: *const core::ffi::c_char);
    pub fn trace_seq_putc(s: *mut trace_seq, c: u8);
    pub fn trace_seq_putmem(s: *mut trace_seq, mem: *const core::ffi::c_void, len: u32);
    pub fn trace_seq_putmem_hex(s: *mut trace_seq, mem: *const core::ffi::c_void, len: u32);
    pub fn trace_seq_path(s: *mut trace_seq, path: *const path) -> i32;
    pub fn trace_seq_bitmask(s: *mut trace_seq, maskp: *const libc::c_ulong, nmaskbits: i32);
    pub fn trace_seq_bitmask_list(s: *mut trace_seq, maskp: *const libc::c_ulong, nmaskbits: i32);
    pub fn trace_seq_hex_dump(
        s: *mut trace_seq,
        prefix_str: *const core::ffi::c_char,
        prefix_type: i32,
        rowsize: i32,
        groupsize: i32,
        buf: *const core::ffi::c_void,
        len: usize,
        ascii: bool,
    ) -> i32;
    pub fn trace_seq_acquire(s: *mut trace_seq, len: u32) -> *mut core::ffi::c_char;
}

#[cfg(not(feature = "CONFIG_TRACING"))]
#[inline]
pub unsafe fn trace_seq_printf(_s: *mut trace_seq, _fmt: *const core::ffi::c_char, ...) {}

#[cfg(not(feature = "CONFIG_TRACING"))]
#[inline]
pub unsafe fn trace_seq_bprintf(_s: *mut trace_seq, _fmt: *const core::ffi::c_char, _binary: *const u32) {}

#[cfg(not(feature = "CONFIG_TRACING"))]
#[inline]
pub unsafe fn trace_seq_bitmask(_s: *mut trace_seq, _maskp: *const libc::c_ulong, _nmaskbits: i32) {}

#[cfg(not(feature = "CONFIG_TRACING"))]
#[inline]
pub unsafe fn trace_seq_bitmask_list(_s: *mut trace_seq, _maskp: *const libc::c_ulong, _nmaskbits: i32) {}

#[cfg(not(feature = "CONFIG_TRACING"))]
#[inline]
pub unsafe fn trace_print_seq(_m: *mut seq_file, _s: *mut trace_seq) -> i32 { 0 }

#[cfg(not(feature = "CONFIG_TRACING"))]
#[inline]
pub unsafe fn trace_seq_to_user(_s: *mut trace_seq, _ubuf: *mut core::ffi::c_char, _cnt: i32) -> i32 { 0 }

#[cfg(not(feature = "CONFIG_TRACING"))]
#[inline]
pub unsafe fn trace_seq_puts(_s: *mut trace_seq, _str_: *const core::ffi::c_char) {}

#[cfg(not(feature = "CONFIG_TRACING"))]
#[inline]
pub unsafe fn trace_seq_putc(_s: *mut trace_seq, _c: u8) {}

#[cfg(not(feature = "CONFIG_TRACING"))]
#[inline]
pub unsafe fn trace_seq_putmem(_s: *mut trace_seq, _mem: *const core::ffi::c_void, _len: u32) {}

#[cfg(not(feature = "CONFIG_TRACING"))]
#[inline]
pub unsafe fn trace_seq_putmem_hex(_s: *mut trace_seq, _mem: *const core::ffi::c_void, _len: u32) {}

#[cfg(not(feature = "CONFIG_TRACING"))]
#[inline]
pub unsafe fn trace_seq_path(_s: *mut trace_seq, _path: *const path) -> i32 { 0 }

#[cfg(not(feature = "CONFIG_TRACING"))]
#[inline]
pub unsafe fn trace_seq_acquire(_s: *mut trace_seq, _len: u32) -> *mut core::ffi::c_char { core::ptr::null_mut() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
