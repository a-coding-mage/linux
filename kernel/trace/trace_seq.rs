// SPDX-License-Identifier: GPL-2.0
/*
 * trace_seq.c
 *
 * Copyright (C) 2008-2014 Red Hat Inc, Steven Rostedt <srostedt@redhat.com>
 *
 * The trace_seq is a handy tool that allows you to pass a descriptor around
 * to a buffer that other functions can write to. It is similar to the
 * seq_file functionality but has some differences.
 *
 * To use it, the trace_seq must be initialized with trace_seq_init().
 * This will set up the counters within the descriptor. You can call
 * trace_seq_init() more than once to reset the trace_seq to start
 * from scratch.
 */

// Kernel dependencies supplied by the surrounding translation unit.

#[inline]
unsafe fn trace_seq_buf_left(s: *mut trace_seq) -> usize {
    seq_buf_buffer_left(&mut (*s).seq)
}

#[inline]
unsafe fn __trace_seq_init(s: *mut trace_seq) {
    if (*s).seq.size == 0 {
        trace_seq_init(s);
    }
}

pub unsafe fn trace_print_seq(m: *mut seq_file, s: *mut trace_seq) -> i32 {
    __trace_seq_init(s);

    let ret = seq_buf_print_seq(m, &mut (*s).seq);

    if ret == 0 {
        trace_seq_init(s);
    }

    ret
}

pub unsafe extern "C" fn trace_seq_printf(s: *mut trace_seq, fmt: *const core::ffi::c_char, ...) {
    let save_len = (*s).seq.len;

    if (*s).full {
        return;
    }

    __trace_seq_init(s);

    // C variadic argument handling is supplied by the target ABI.
    let mut ap = core::mem::MaybeUninit::<va_list>::uninit();
    seq_buf_vprintf(&mut (*s).seq, fmt, ap.as_mut_ptr());

    if seq_buf_has_overflowed(&(*s).seq) {
        (*s).seq.len = save_len;
        (*s).full = true;
    }
}

pub unsafe fn trace_seq_bitmask(
    s: *mut trace_seq,
    maskp: *const core::ffi::c_ulong,
    nmaskbits: i32,
) {
    let save_len = (*s).seq.len;

    if (*s).full {
        return;
    }

    __trace_seq_init(s);
    seq_buf_printf(&mut (*s).seq, b"%*pb\0".as_ptr() as *const core::ffi::c_char, nmaskbits, maskp);

    if seq_buf_has_overflowed(&(*s).seq) {
        (*s).seq.len = save_len;
        (*s).full = true;
    }
}

pub unsafe fn trace_seq_bitmask_list(
    s: *mut trace_seq,
    maskp: *const core::ffi::c_ulong,
    nmaskbits: i32,
) {
    let save_len = (*s).seq.len;

    if (*s).full {
        return;
    }

    __trace_seq_init(s);
    seq_buf_printf(&mut (*s).seq, b"%*pbl\0".as_ptr() as *const core::ffi::c_char, nmaskbits, maskp);

    if seq_buf_has_overflowed(&(*s).seq) {
        (*s).seq.len = save_len;
        (*s).full = true;
    }
}

pub unsafe fn trace_seq_vprintf(
    s: *mut trace_seq,
    fmt: *const core::ffi::c_char,
    args: va_list,
) {
    let save_len = (*s).seq.len;

    if (*s).full {
        return;
    }

    __trace_seq_init(s);
    seq_buf_vprintf(&mut (*s).seq, fmt, args);

    if seq_buf_has_overflowed(&(*s).seq) {
        (*s).seq.len = save_len;
        (*s).full = true;
    }
}

pub unsafe fn trace_seq_bprintf(
    s: *mut trace_seq,
    fmt: *const core::ffi::c_char,
    binary: *const u32,
) {
    let save_len = (*s).seq.len;

    if (*s).full {
        return;
    }

    __trace_seq_init(s);
    seq_buf_bprintf(&mut (*s).seq, fmt, binary);

    if seq_buf_has_overflowed(&(*s).seq) {
        (*s).seq.len = save_len;
        (*s).full = true;
    }
}

pub unsafe fn trace_seq_puts(s: *mut trace_seq, str_: *const core::ffi::c_char) {
    let len = strlen(str_);

    if (*s).full {
        return;
    }

    __trace_seq_init(s);

    if len > trace_seq_buf_left(s) {
        (*s).full = true;
        return;
    }

    seq_buf_putmem(&mut (*s).seq, str_ as *const core::ffi::c_void, len);
}

pub unsafe fn trace_seq_putc(s: *mut trace_seq, c: u8) {
    if (*s).full {
        return;
    }

    __trace_seq_init(s);

    if trace_seq_buf_left(s) < 1 {
        (*s).full = true;
        return;
    }

    seq_buf_putc(&mut (*s).seq, c);
}

pub unsafe fn trace_seq_putmem(s: *mut trace_seq, mem: *const core::ffi::c_void, len: u32) {
    if (*s).full {
        return;
    }

    __trace_seq_init(s);

    if len as usize > trace_seq_buf_left(s) {
        (*s).full = true;
        return;
    }

    seq_buf_putmem(&mut (*s).seq, mem, len as usize);
}

pub unsafe fn trace_seq_putmem_hex(s: *mut trace_seq, mem: *const core::ffi::c_void, len: u32) {
    let save_len = (*s).seq.len;

    if (*s).full {
        return;
    }

    __trace_seq_init(s);

    if len.wrapping_mul(2) as usize > trace_seq_buf_left(s) {
        (*s).full = true;
        return;
    }

    seq_buf_putmem_hex(&mut (*s).seq, mem, len as usize);

    if seq_buf_has_overflowed(&(*s).seq) {
        (*s).seq.len = save_len;
        (*s).full = true;
    }
}

pub unsafe fn trace_seq_path(s: *mut trace_seq, path: *const path) -> i32 {
    let save_len = (*s).seq.len;

    if (*s).full {
        return 0;
    }

    __trace_seq_init(s);

    if trace_seq_buf_left(s) < 1 {
        (*s).full = true;
        return 0;
    }

    seq_buf_path(&mut (*s).seq, path, b"\n\0".as_ptr() as *const core::ffi::c_char);

    if seq_buf_has_overflowed(&(*s).seq) {
        (*s).seq.len = save_len;
        (*s).full = true;
        return 0;
    }

    1
}

pub unsafe fn trace_seq_to_user(
    s: *mut trace_seq,
    ubuf: *mut core::ffi::c_char,
    cnt: i32,
) -> i32 {
    __trace_seq_init(s);
    let ret = seq_buf_to_user(&mut (*s).seq, ubuf, (*s).readpos, cnt);
    if ret > 0 {
        (*s).readpos += ret;
    }
    ret
}

pub unsafe fn trace_seq_hex_dump(
    s: *mut trace_seq,
    prefix_str: *const core::ffi::c_char,
    prefix_type: i32,
    rowsize: i32,
    groupsize: i32,
    buf: *const core::ffi::c_void,
    len: usize,
    ascii: bool,
) -> i32 {
    let save_len = (*s).seq.len;

    if (*s).full {
        return 0;
    }

    __trace_seq_init(s);

    if trace_seq_buf_left(s) < 1 {
        (*s).full = true;
        return 0;
    }

    seq_buf_hex_dump(&mut (*s).seq, prefix_str, prefix_type, rowsize, groupsize, buf, len, ascii);

    if seq_buf_has_overflowed(&(*s).seq) {
        (*s).seq.len = save_len;
        (*s).full = true;
        return 0;
    }

    1
}

pub unsafe fn trace_seq_acquire(s: *mut trace_seq, len: u32) -> *mut core::ffi::c_char {
    let ret = trace_seq_buffer_ptr(s);

    if seq_buf_buffer_left(&(*s).seq) >= len as usize {
        seq_buf_commit(&mut (*s).seq, len as usize);
    }

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
