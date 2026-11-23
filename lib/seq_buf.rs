// SPDX-License-Identifier: GPL-2.0
/*
 * seq_buf.c
 *
 * Copyright (C) 2014 Red Hat Inc, Steven Rostedt <srostedt@redhat.com>
 *
 * The seq_buf is a handy tool that allows you to pass a descriptor around
 * to a buffer that other functions can write to. It is similar to the
 * seq_file functionality but has some differences.
 *
 * To use it, the seq_buf must be initialized with seq_buf_init().
 * This will set up the counters within the descriptor. You can call
 * seq_buf_init() more than once to reset the seq_buf to start
 * from scratch.
 */

// Dependencies: linux/bug.h, linux/err.h, linux/hex.h, linux/printk.h,
// linux/seq_buf.h, linux/seq_file.h, linux/sprintf.h, linux/string.h,
// linux/uaccess.h.
//
// C-variadic definitions and va_list parameters use the compiler's VaList
// ABI, admitted for this object by lib/Makefile (RUST_SEQ_BUF requires the
// inspected rustc 1.85.1, as RUST_PANIC does).

use core::cmp::min;
use core::ffi::VaList;
use core::ptr::{addr_of, copy_nonoverlapping, null_mut};
use kernel::bindings::{
    _printk, copy_to_user, d_path, hex_dump_to_buffer, path, seq_file, seq_mangle_path, seq_write,
    strchr, strlen, EBUSY, EFAULT,
};
use kernel::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

/// `struct seq_buf` from linux/seq_buf.h, which the generated bindings only
/// see as an opaque declaration; lib/seq_buf_helpers.c asserts the layout.
#[repr(C)]
pub struct SeqBuf {
    buffer: *mut c_char,
    size: usize,
    len: usize,
}

extern "C" {
    /// `const char hex_asc[]` ("0123456789abcdef"), owned by the hexdump unit.
    static hex_asc: [c_char; 0];

    #[link_name = "vsnprintf"]
    fn native_vsnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, args: VaList<'_, '_>) -> c_int;

    fn lupos_seq_buf_warn_on();
    fn lupos_seq_buf_warn_on_once();
    fn lupos_seq_buf_bug();
}

#[cfg(CONFIG_BINARY_PRINTF)]
use kernel::bindings::bstr_printf;

/// `DUMP_PREFIX_ADDRESS` and `DUMP_PREFIX_OFFSET` from linux/printk.h;
/// lib/seq_buf_helpers.c asserts them.
const DUMP_PREFIX_ADDRESS: c_int = 1;
const DUMP_PREFIX_OFFSET: c_int = 2;
/// MAX_ERRNO from linux/err.h, for IS_ERR().
const MAX_ERRNO: c_ulong = 4095;

/// WARN_ON(condition), returning the condition.
fn warn_on(condition: bool) -> bool {
    if condition {
        // SAFETY: the native warning has no preconditions.
        unsafe { lupos_seq_buf_warn_on() };
    }
    condition
}

/// hex_asc[x & 0x0f] / hex_asc[(x & 0xf0) >> 4].
fn hex_asc_at(nibble: u8) -> c_char {
    // SAFETY: hex_asc holds 16 digits; every caller masks to a nibble.
    unsafe { *addr_of!(hex_asc).cast::<c_char>().add(usize::from(nibble & 0x0f)) }
}

fn hex_asc_lo(x: u8) -> c_char {
    hex_asc_at(x & 0x0f)
}

fn hex_asc_hi(x: u8) -> c_char {
    hex_asc_at((x & 0xf0) >> 4)
}

/// # Safety
/// `s` points to a valid `struct seq_buf`.
unsafe fn seq_buf_has_overflowed(s: *mut SeqBuf) -> bool {
    // SAFETY: s is valid.
    unsafe { (*s).len > (*s).size }
}

/// # Safety
/// `s` points to a valid `struct seq_buf`.
unsafe fn seq_buf_set_overflow(s: *mut SeqBuf) {
    // SAFETY: s is valid.
    unsafe { (*s).len = (*s).size.wrapping_add(1) };
}

/// # Safety
/// `s` points to a valid `struct seq_buf`.
unsafe fn seq_buf_buffer_left(s: *mut SeqBuf) -> c_uint {
    // SAFETY: s is valid.
    unsafe {
        if seq_buf_has_overflowed(s) {
            return 0;
        }

        (*s).size.wrapping_sub((*s).len) as c_uint
    }
}

/// How much buffer was written?
///
/// # Safety
/// `s` points to a valid `struct seq_buf`.
unsafe fn seq_buf_used(s: *mut SeqBuf) -> c_uint {
    // SAFETY: s is valid.
    unsafe { min((*s).len, (*s).size) as c_uint }
}

/// # Safety
/// `s` points to a valid `struct seq_buf` with a nonzero-size buffer.
unsafe fn seq_buf_str(s: *mut SeqBuf) -> *const c_char {
    // SAFETY: s is valid; the NUL lands inside the buffer.
    unsafe {
        if warn_on((*s).size == 0) {
            return c"".as_ptr().cast();
        }

        if seq_buf_buffer_left(s) != 0 {
            *(*s).buffer.add((*s).len) = 0;
        } else {
            *(*s).buffer.add((*s).size.wrapping_sub(1)) = 0;
        }

        (*s).buffer
    }
}

/// # Safety
/// `s` points to a valid `struct seq_buf` and `bufp` to writable storage.
unsafe fn seq_buf_get_buf(s: *mut SeqBuf, bufp: *mut *mut c_char) -> usize {
    // SAFETY: s is valid; the returned region lies inside its buffer.
    unsafe {
        warn_on((*s).len > (*s).size.wrapping_add(1));

        if (*s).len < (*s).size {
            *bufp = (*s).buffer.add((*s).len);
            return (*s).size.wrapping_sub((*s).len);
        }

        *bufp = null_mut();
        0
    }
}

/// # Safety
/// `s` points to a valid `struct seq_buf`.
unsafe fn seq_buf_commit(s: *mut SeqBuf, num: c_int) {
    // SAFETY: s is valid.
    unsafe {
        if num < 0 {
            seq_buf_set_overflow(s);
        } else {
            /* num must be negative on overflow */
            if (*s).len.wrapping_add(num as usize) > (*s).size {
                lupos_seq_buf_bug();
            }
            (*s).len = (*s).len.wrapping_add(num as usize);
        }
    }
}

/**
 * seq_buf_can_fit - can the new data fit in the current buffer?
 * @s: the seq_buf descriptor
 * @len: The length to see if it can fit in the current buffer
 *
 * Returns: true if there's enough unused space in the seq_buf buffer
 * to fit the amount of new data according to @len.
 */
/// # Safety
/// `s` points to a valid `struct seq_buf`.
unsafe fn seq_buf_can_fit(s: *mut SeqBuf, len: usize) -> bool {
    // SAFETY: s is valid.
    unsafe { (*s).len.wrapping_add(len) <= (*s).size }
}

/**
 * seq_buf_print_seq - move the contents of seq_buf into a seq_file
 * @m: the seq_file descriptor that is the destination
 * @s: the seq_buf descriptor that is the source.
 *
 * Returns: zero on success, non-zero otherwise.
 */
/// # Safety
/// `m` and `s` point to valid descriptors.
#[no_mangle]
pub unsafe extern "C" fn seq_buf_print_seq(m: *mut seq_file, s: *mut SeqBuf) -> c_int {
    // SAFETY: forwarded caller contract.
    unsafe {
        let len = seq_buf_used(s);

        seq_write(m, (*s).buffer.cast::<c_void>(), len as usize)
    }
}

/**
 * seq_buf_vprintf - sequence printing of information.
 * @s: seq_buf descriptor
 * @fmt: printf format string
 * @args: va_list of arguments from a printf() type function
 *
 * Writes a vnprintf() format into the sequence buffer.
 *
 * Returns: zero on success, -1 on overflow.
 */
/// # Safety
/// `s` is a valid descriptor and `fmt`/`args` a valid printf format and
/// argument list.
#[no_mangle]
pub unsafe extern "C" fn seq_buf_vprintf(s: *mut SeqBuf, fmt: *const c_char, args: VaList<'_, '_>) -> c_int {
    // SAFETY: the formatted output is bounded by the remaining buffer size.
    unsafe {
        warn_on((*s).size == 0);

        if (*s).len < (*s).size {
            let len = native_vsnprintf((*s).buffer.add((*s).len), (*s).size.wrapping_sub((*s).len), fmt, args);
            if (*s).len.wrapping_add(len as usize) < (*s).size {
                (*s).len = (*s).len.wrapping_add(len as usize);
                return 0;
            }
        }
        seq_buf_set_overflow(s);
    }
    -1
}

/**
 * seq_buf_printf - sequence printing of information
 * @s: seq_buf descriptor
 * @fmt: printf format string
 *
 * Writes a printf() format into the sequence buffer.
 *
 * Returns: zero on success, -1 on overflow.
 */
/// # Safety
/// `s` is a valid descriptor and `fmt` a printf format matching the
/// variadic arguments.
#[no_mangle]
pub unsafe extern "C" fn seq_buf_printf(s: *mut SeqBuf, fmt: *const c_char, mut args: ...) -> c_int {
    // SAFETY: va_start/va_end bracket this call, as in the C body.
    unsafe { seq_buf_vprintf(s, fmt, args.as_va_list()) }
}
// EXPORT_SYMBOL_GPL(seq_buf_printf);

/**
 * seq_buf_do_printk - printk() seq_buf line by line
 * @s: seq_buf descriptor
 * @lvl: printk level
 *
 * printk()-s a multi-line sequential buffer line by line. The function
 * makes sure that the buffer in @s is NUL-terminated and safe to read
 * as a string.
 */
/// # Safety
/// `s` is a valid descriptor and `lvl` a NUL-terminated printk level.
#[no_mangle]
pub unsafe extern "C" fn seq_buf_do_printk(s: *mut SeqBuf, lvl: *const c_char) {
    // SAFETY: seq_buf_str() NUL-terminates the buffer before it is scanned.
    unsafe {
        if (*s).size == 0 || (*s).len == 0 {
            return;
        }

        let mut start = seq_buf_str(s);
        loop {
            let mut lf: *const c_char = strchr(start, c_int::from(b'\n'));
            if lf.is_null() {
                break;
            }
            let len = lf.offset_from(start).wrapping_add(1) as c_int;

            _printk(c"%s%.*s".as_ptr().cast(), lvl, len, start);
            lf = lf.add(1);
            start = lf;
        }

        /* No trailing LF */
        if start < (*s).buffer.add((*s).len).cast_const() {
            _printk(c"%s%s\n".as_ptr().cast(), lvl, start);
        }
    }
}
// EXPORT_SYMBOL_GPL(seq_buf_do_printk);

/**
 * seq_buf_bprintf - Write the printf string from binary arguments
 * @s: seq_buf descriptor
 * @fmt: The format string for the @binary arguments
 * @binary: The binary arguments for @fmt.
 *
 * When recording in a fast path, a printf may be recorded with just
 * saving the format and the arguments as they were passed to the
 * function, instead of wasting cycles converting the arguments into
 * ASCII characters. Instead, the arguments are saved in a 32 bit
 * word array that is defined by the format string constraints.
 *
 * This function will take the format and the binary array and finish
 * the conversion into the ASCII string within the buffer.
 *
 * Returns: zero on success, -1 on overflow.
 */
/// # Safety
/// `s` is a valid descriptor and `fmt`/`binary` a recorded binary printf.
#[cfg(CONFIG_BINARY_PRINTF)]
#[no_mangle]
pub unsafe extern "C" fn seq_buf_bprintf(s: *mut SeqBuf, fmt: *const c_char, binary: *const u32) -> c_int {
    // SAFETY: the formatted output is bounded by the remaining buffer size.
    unsafe {
        let len = seq_buf_buffer_left(s);

        warn_on((*s).size == 0);

        if (*s).len < (*s).size {
            let ret = bstr_printf((*s).buffer.add((*s).len), len as usize, fmt, binary);
            if (*s).len.wrapping_add(ret as usize) < (*s).size {
                (*s).len = (*s).len.wrapping_add(ret as usize);
                return 0;
            }
        }
        seq_buf_set_overflow(s);
    }
    -1
}

/**
 * seq_buf_puts - sequence printing of simple string
 * @s: seq_buf descriptor
 * @str: simple string to record
 *
 * Copy a simple string into the sequence buffer.
 *
 * Returns: zero on success, -1 on overflow.
 */
/// # Safety
/// `s` is a valid descriptor and `str` a NUL-terminated string.
#[no_mangle]
pub unsafe extern "C" fn seq_buf_puts(s: *mut SeqBuf, str: *const c_char) -> c_int {
    // SAFETY: the copy, including the NUL, fits the remaining buffer.
    unsafe {
        let mut len = strlen(str);

        warn_on((*s).size == 0);

        /* Add 1 to len for the trailing null byte which must be there */
        len = len.wrapping_add(1);

        if seq_buf_can_fit(s, len) {
            copy_nonoverlapping(str, (*s).buffer.add((*s).len), len);
            /* Don't count the trailing null byte against the capacity */
            (*s).len = (*s).len.wrapping_add(len.wrapping_sub(1));
            return 0;
        }
        seq_buf_set_overflow(s);
    }
    -1
}
// EXPORT_SYMBOL_GPL(seq_buf_puts);

/**
 * seq_buf_putc - sequence printing of simple character
 * @s: seq_buf descriptor
 * @c: simple character to record
 *
 * Copy a single character into the sequence buffer.
 *
 * Returns: zero on success, -1 on overflow.
 */
/// # Safety
/// `s` is a valid descriptor.
#[no_mangle]
pub unsafe extern "C" fn seq_buf_putc(s: *mut SeqBuf, c: u8) -> c_int {
    // SAFETY: the byte fits the remaining buffer.
    unsafe {
        warn_on((*s).size == 0);

        if seq_buf_can_fit(s, 1) {
            *(*s).buffer.add((*s).len) = c;
            (*s).len = (*s).len.wrapping_add(1);
            return 0;
        }
        seq_buf_set_overflow(s);
    }
    -1
}
// EXPORT_SYMBOL_GPL(seq_buf_putc);

/**
 * seq_buf_putmem - write raw data into the sequence buffer
 * @s: seq_buf descriptor
 * @mem: The raw memory to copy into the buffer
 * @len: The length of the raw memory to copy (in bytes)
 *
 * There may be cases where raw memory needs to be written into the
 * buffer and a strcpy() would not work. Using this function allows
 * for such cases.
 *
 * Returns: zero on success, -1 on overflow.
 */
/// # Safety
/// `s` is a valid descriptor and `mem` readable for `len` bytes.
#[no_mangle]
pub unsafe extern "C" fn seq_buf_putmem(s: *mut SeqBuf, mem: *const c_void, len: c_uint) -> c_int {
    // SAFETY: the copy fits the remaining buffer.
    unsafe {
        warn_on((*s).size == 0);

        if seq_buf_can_fit(s, len as usize) {
            copy_nonoverlapping(mem.cast::<u8>(), (*s).buffer.add((*s).len), len as usize);
            (*s).len = (*s).len.wrapping_add(len as usize);
            return 0;
        }
        seq_buf_set_overflow(s);
    }
    -1
}

const MAX_MEMHEX_BYTES: c_uint = 8;
const HEX_CHARS: usize = MAX_MEMHEX_BYTES as usize * 2 + 1;

/**
 * seq_buf_putmem_hex - write raw memory into the buffer in ASCII hex
 * @s: seq_buf descriptor
 * @mem: The raw memory to write its hex ASCII representation of
 * @len: The length of the raw memory to copy (in bytes)
 *
 * This is similar to seq_buf_putmem() except instead of just copying the
 * raw memory into the buffer it writes its ASCII representation of it
 * in hex characters.
 *
 * Returns: zero on success, -1 on overflow.
 */
/// # Safety
/// `s` is a valid descriptor and `mem` readable for `len` bytes.
#[no_mangle]
pub unsafe extern "C" fn seq_buf_putmem_hex(s: *mut SeqBuf, mem: *const c_void, mut len: c_uint) -> c_int {
    let mut hex = [0u8; HEX_CHARS];
    let mut data = mem.cast::<u8>();

    // SAFETY: data stays within the caller's `len` readable bytes; at most
    // HEX_CHARS bytes of `hex` are written per chunk.
    unsafe {
        warn_on((*s).size == 0);

        const _: () = assert!((MAX_MEMHEX_BYTES as usize) * 2 < HEX_CHARS);

        while len != 0 {
            let start_len = min(len, MAX_MEMHEX_BYTES);
            let mut j: c_int = 0;

            let hexp = hex.as_mut_ptr();
            #[cfg(target_endian = "big")]
            for i in 0..start_len as usize {
                *hexp.add(j as usize) = hex_asc_hi(*data.add(i));
                j = j.wrapping_add(1);
                *hexp.add(j as usize) = hex_asc_lo(*data.add(i));
                j = j.wrapping_add(1);
            }
            #[cfg(target_endian = "little")]
            for i in (0..start_len as usize).rev() {
                *hexp.add(j as usize) = hex_asc_hi(*data.add(i));
                j = j.wrapping_add(1);
                *hexp.add(j as usize) = hex_asc_lo(*data.add(i));
                j = j.wrapping_add(1);
            }
            if j == 0 || (j / 2) as c_uint > len {
                lupos_seq_buf_warn_on_once();
                break;
            }

            /* j increments twice per loop */
            *hexp.add(j as usize) = b' ';
            j = j.wrapping_add(1);

            seq_buf_putmem(s, hex.as_ptr().cast::<c_void>(), j as c_uint);
            if seq_buf_has_overflowed(s) {
                return -1;
            }

            len = len.wrapping_sub(start_len);
            data = data.add(start_len as usize);
        }
    }
    0
}
// EXPORT_SYMBOL_GPL(seq_buf_putmem_hex);

/**
 * seq_buf_path - copy a path into the sequence buffer
 * @s: seq_buf descriptor
 * @path: path to write into the sequence buffer.
 * @esc: set of characters to escape in the output
 *
 * Write a path name into the sequence buffer.
 *
 * Returns: the number of written bytes on success, -1 on overflow.
 */
/// # Safety
/// `s` is a valid descriptor, `path` a valid path and `esc` a
/// NUL-terminated escape set.
#[no_mangle]
pub unsafe extern "C" fn seq_buf_path(s: *mut SeqBuf, path: *const path, esc: *const c_char) -> c_int {
    let mut buf: *mut c_char = null_mut();

    // SAFETY: d_path() and seq_mangle_path() write inside the free region
    // returned by seq_buf_get_buf().
    unsafe {
        let size = seq_buf_get_buf(s, &mut buf);
        let mut res: c_int = -1;

        warn_on((*s).size == 0);

        if size != 0 {
            let p = d_path(path, buf, size as c_int);
            // !IS_ERR(p)
            if (p as c_ulong) < MAX_ERRNO.wrapping_neg() {
                let end = seq_mangle_path(buf, p, esc);
                if !end.is_null() {
                    res = end.offset_from(buf) as c_int;
                }
            }
        }
        seq_buf_commit(s, res);

        res
    }
}

/**
 * seq_buf_to_user - copy the sequence buffer to user space
 * @s: seq_buf descriptor
 * @ubuf: The userspace memory location to copy to
 * @start: The first byte in the buffer to copy
 * @cnt: The amount to copy
 *
 * Copies the sequence buffer into the userspace memory pointed to
 * by @ubuf. It starts from @start and writes up to @cnt characters
 * or until it reaches the end of the content in the buffer (@s->len),
 * whichever comes first.
 *
 * Returns:
 * On success, it returns a positive number of the number of bytes
 * it copied.
 *
 * On failure it returns -EBUSY if all of the content in the
 * sequence has been already read, which includes nothing in the
 * sequence (@s->len == @start).
 *
 * Returns -EFAULT if the copy to userspace fails.
 */
/// # Safety
/// `s` is a valid descriptor; `ubuf` is a user pointer checked by
/// copy_to_user().
#[no_mangle]
pub unsafe extern "C" fn seq_buf_to_user(s: *mut SeqBuf, ubuf: *mut c_char, start: usize, mut cnt: c_int) -> c_int {
    if cnt == 0 {
        return 0;
    }

    // SAFETY: `start` is below the used length, so the source lies in the
    // buffer; copy_to_user() validates the user range.
    unsafe {
        // The int/size_t mixing follows the C usual arithmetic conversions.
        let mut len = seq_buf_used(s) as c_int;

        if len as usize <= start {
            return -(EBUSY as c_int);
        }

        len = (len as usize).wrapping_sub(start) as c_int;
        if cnt > len {
            cnt = len;
        }
        let ret = copy_to_user(
            ubuf.cast::<c_void>(),
            (*s).buffer.add(start).cast::<c_void>(),
            cnt as c_ulong,
        ) as c_int;
        if ret == cnt {
            return -(EFAULT as c_int);
        }

        cnt.wrapping_sub(ret)
    }
}

/**
 * seq_buf_hex_dump - print formatted hex dump into the sequence buffer
 * @s: seq_buf descriptor
 * @prefix_str: string to prefix each line with;
 *  caller supplies trailing spaces for alignment if desired
 * @prefix_type: controls whether prefix of an offset, address, or none
 *  is printed (%DUMP_PREFIX_OFFSET, %DUMP_PREFIX_ADDRESS, %DUMP_PREFIX_NONE)
 * @rowsize: number of bytes to print per line; must be 16 or 32
 * @groupsize: number of bytes to print at a time (1, 2, 4, 8; default = 1)
 * @buf: data blob to dump
 * @len: number of bytes in the @buf
 * @ascii: include ASCII after the hex output
 *
 * Function is an analogue of print_hex_dump() and thus has similar interface.
 *
 * linebuf size is maximal length for one line.
 * 32 * 3 - maximum bytes per line, each printed into 2 chars + 1 for
 *	separating space
 * 2 - spaces separating hex dump and ascii representation
 * 32 - ascii representation
 * 1 - terminating '\0'
 *
 * Returns: zero on success, -1 on overflow.
 */
/// # Safety
/// `s` is a valid descriptor, `prefix_str` NUL-terminated and `buf`
/// readable for `len` bytes.
#[no_mangle]
pub unsafe extern "C" fn seq_buf_hex_dump(
    s: *mut SeqBuf,
    prefix_str: *const c_char,
    prefix_type: c_int,
    mut rowsize: c_int,
    groupsize: c_int,
    buf: *const c_void,
    len: usize,
    ascii: bool,
) -> c_int {
    let ptr = buf.cast::<u8>();
    let mut remaining = len as c_int;
    let mut linebuf = [0u8; 32 * 3 + 2 + 32 + 1];

    if rowsize != 16 && rowsize != 32 {
        rowsize = 16;
    }

    let mut i: c_int = 0;
    // SAFETY: each row stays within the caller's `len` readable bytes and
    // hex_dump_to_buffer() is bounded by sizeof(linebuf).
    unsafe {
        while (i as usize) < len {
            let linelen = min(remaining, rowsize);
            remaining = remaining.wrapping_sub(rowsize);

            hex_dump_to_buffer(
                ptr.wrapping_offset(i as isize).cast::<c_void>(),
                linelen as usize,
                rowsize,
                groupsize,
                linebuf.as_mut_ptr(),
                linebuf.len(),
                ascii,
            );

            let ret = match prefix_type {
                DUMP_PREFIX_ADDRESS => seq_buf_printf(
                    s,
                    c"%s%p: %s\n".as_ptr().cast(),
                    prefix_str,
                    ptr.wrapping_offset(i as isize),
                    linebuf.as_ptr(),
                ),
                DUMP_PREFIX_OFFSET => {
                    seq_buf_printf(s, c"%s%.8x: %s\n".as_ptr().cast(), prefix_str, i, linebuf.as_ptr())
                }
                _ => seq_buf_printf(s, c"%s%s\n".as_ptr().cast(), prefix_str, linebuf.as_ptr()),
            };
            if ret != 0 {
                return ret;
            }
            i = i.wrapping_add(rowsize);
        }
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
