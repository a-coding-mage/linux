// SPDX-License-Identifier: GPL-2.0
/*
 * seq_buf.c
 *
 * Copyright (C) 2014 Red Hat Inc, Steven Rostedt <srostedt@redhat.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct seq_buf {
    pub buffer: *mut c_char,
    pub size: usize,
    pub len: usize,
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct path {
    _private: [u8; 0],
}

extern "C" {
    fn seq_write(m: *mut seq_file, data: *const c_char, len: c_uint) -> c_int;
    fn vsnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, args: *mut c_void) -> c_int;
    fn seq_buf_set_overflow(s: *mut seq_buf);
    fn seq_buf_used(s: *const seq_buf) -> c_uint;
    fn seq_buf_str(s: *mut seq_buf) -> *mut c_char;
    fn seq_buf_buffer_left(s: *const seq_buf) -> c_uint;
    fn seq_buf_has_overflowed(s: *const seq_buf) -> bool;
    fn bstr_printf(buf: *mut c_char, size: c_uint, fmt: *const c_char, binary: *const u32) -> c_int;
    fn printk(fmt: *const c_char, ...);
    fn strlen(s: *const c_char) -> usize;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn d_path(path: *const path, buf: *mut c_char, size: usize) -> *mut c_char;
    fn seq_mangle_path(buf: *mut c_char, p: *mut c_char, esc: *const c_char) -> *mut c_char;
    fn seq_buf_get_buf(s: *mut seq_buf, buf: *mut *mut c_char) -> usize;
    fn seq_buf_commit(s: *mut seq_buf, num: c_int);
    fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> usize;
    fn hex_dump_to_buffer(src: *const c_void, len: c_int, rowsize: c_int,
                          groupsize: c_int, linebuf: *mut u8, linebuflen: usize,
                          ascii: bool);
    fn hex_asc_hi(n: u8) -> u8;
    fn hex_asc_lo(n: u8) -> u8;
}

const MAX_MEMHEX_BYTES: usize = 8;
const HEX_CHARS: usize = MAX_MEMHEX_BYTES * 2 + 1;
const EBUSY: c_int = 16;
const EFAULT: c_int = 14;
const DUMP_PREFIX_ADDRESS: c_int = 1;
const DUMP_PREFIX_OFFSET: c_int = 2;

#[inline]
unsafe fn seq_buf_can_fit(s: *const seq_buf, len: usize) -> bool {
    (*s).len + len <= (*s).size
}

pub unsafe fn seq_buf_print_seq(m: *mut seq_file, s: *mut seq_buf) -> c_int {
    let len = seq_buf_used(s);
    seq_write(m, (*s).buffer, len)
}

pub unsafe fn seq_buf_vprintf(s: *mut seq_buf, fmt: *const c_char, args: *mut c_void) -> c_int {
    let mut len: c_int;
    if (*s).size != 0 && (*s).len < (*s).size {
        len = vsnprintf((*s).buffer.add((*s).len), (*s).size - (*s).len, fmt, args);
        if (*s).len + len as usize < (*s).size {
            (*s).len += len as usize;
            return 0;
        }
    }
    seq_buf_set_overflow(s);
    -1
}

pub unsafe extern "C" fn seq_buf_printf(s: *mut seq_buf, fmt: *const c_char, mut args: ...) -> c_int {
    seq_buf_vprintf(s, fmt, &mut args as *mut _ as *mut c_void)
}

pub unsafe fn seq_buf_do_printk(s: *mut seq_buf, lvl: *const c_char) {
    if (*s).size == 0 || (*s).len == 0 { return; }
    let mut start = seq_buf_str(s);
    while {
        let lf = strchr(start, b'\n' as c_int);
        if lf.is_null() { false } else {
            let len = lf.offset_from(start) as c_int + 1;
            printk(b"%s%.*s\0".as_ptr() as *const c_char, lvl, len, start);
            start = lf.add(1);
            true
        }
    } {}
    if start < (*s).buffer.add((*s).len) {
        printk(b"%s%s\n\0".as_ptr() as *const c_char, lvl, start);
    }
}

pub unsafe fn seq_buf_puts(s: *mut seq_buf, str_: *const c_char) -> c_int {
    let len = strlen(str_) + 1;
    if seq_buf_can_fit(s, len) {
        memcpy((*s).buffer.add((*s).len) as *mut c_void, str_ as *const c_void, len);
        (*s).len += len - 1;
        return 0;
    }
    seq_buf_set_overflow(s); -1
}

pub unsafe fn seq_buf_putc(s: *mut seq_buf, c: u8) -> c_int {
    if seq_buf_can_fit(s, 1) {
        *(*s).buffer.add((*s).len) as *mut u8 = c;
        (*s).len += 1;
        return 0;
    }
    seq_buf_set_overflow(s); -1
}

pub unsafe fn seq_buf_putmem(s: *mut seq_buf, mem: *const c_void, len: c_uint) -> c_int {
    if seq_buf_can_fit(s, len as usize) {
        memcpy((*s).buffer.add((*s).len) as *mut c_void, mem, len as usize);
        (*s).len += len as usize;
        return 0;
    }
    seq_buf_set_overflow(s); -1
}

pub unsafe fn seq_buf_putmem_hex(s: *mut seq_buf, mem: *const c_void, mut len: c_uint) -> c_int {
    let mut hex = [0u8; HEX_CHARS];
    let mut data = mem as *const u8;
    while len != 0 {
        let start_len = core::cmp::min(len as usize, MAX_MEMHEX_BYTES);
        let mut j = 0usize;
        let mut i = start_len as isize - 1;
        while i >= 0 {
            *hex.get_unchecked_mut(j) = hex_asc_hi(*data.add(i as usize)); j += 1;
            *hex.get_unchecked_mut(j) = hex_asc_lo(*data.add(i as usize)); j += 1;
            i -= 1;
        }
        *hex.get_unchecked_mut(j) = b' '; j += 1;
        seq_buf_putmem(s, hex.as_ptr() as *const c_void, j as c_uint);
        if seq_buf_has_overflowed(s) { return -1; }
        len -= start_len as c_uint; data = data.add(start_len);
    }
    0
}

pub unsafe fn seq_buf_path(s: *mut seq_buf, p: *const path, esc: *const c_char) -> c_int {
    let mut buf = core::ptr::null_mut();
    let size = seq_buf_get_buf(s, &mut buf);
    let mut res = -1;
    if size != 0 {
        let out = d_path(p, buf, size);
        if !out.is_null() {
            let end = seq_mangle_path(buf, out, esc);
            if !end.is_null() { res = end.offset_from(buf) as c_int; }
        }
    }
    seq_buf_commit(s, res); res
}

pub unsafe fn seq_buf_to_user(s: *mut seq_buf, ubuf: *mut c_char, start: usize, mut cnt: c_int) -> c_int {
    if cnt == 0 { return 0; }
    let mut len = seq_buf_used(s) as usize;
    if len <= start { return -EBUSY; }
    len -= start; if cnt as usize > len { cnt = len as c_int; }
    let ret = copy_to_user(ubuf as *mut c_void, (*s).buffer.add(start) as *const c_void, cnt as usize);
    if ret == cnt as usize { return -EFAULT; }
    cnt - ret as c_int
}

pub unsafe fn seq_buf_hex_dump(s: *mut seq_buf, prefix_str: *const c_char, prefix_type: c_int,
                               mut rowsize: c_int, groupsize: c_int, buf: *const c_void,
                               len: usize, ascii: bool) -> c_int {
    if rowsize != 16 && rowsize != 32 { rowsize = 16; }
    let mut remaining = len as c_int; let mut i = 0usize;
    let mut linebuf = [0u8; 32 * 3 + 2 + 32 + 1];
    while i < len {
        let linelen = core::cmp::min(remaining, rowsize); remaining -= rowsize;
        hex_dump_to_buffer((buf as *const u8).add(i) as *const c_void, linelen, rowsize, groupsize,
                           linebuf.as_mut_ptr(), linebuf.len(), ascii);
        let ret = match prefix_type {
            DUMP_PREFIX_ADDRESS => seq_buf_printf(s, b"%s%p: %s\n\0".as_ptr() as *const c_char, prefix_str, (buf as *const u8).add(i), linebuf.as_ptr()),
            DUMP_PREFIX_OFFSET => seq_buf_printf(s, b"%s%.8x: %s\n\0".as_ptr() as *const c_char, prefix_str, i, linebuf.as_ptr()),
            _ => seq_buf_printf(s, b"%s%s\n\0".as_ptr() as *const c_char, prefix_str, linebuf.as_ptr()),
        };
        if ret != 0 { return ret; }
        i += rowsize as usize;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
