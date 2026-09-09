/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Trace sequences are used to allow a function to call several other functions
 * to create a string of data to use.
 */

/**
 * struct seq_buf - seq buffer structure
 * @buffer: pointer to the buffer
 * @size:   size of the buffer
 * @len:    the amount of data inside the buffer
 */
#[repr(C)]
pub struct seq_buf {
    pub buffer: *mut ::core::ffi::c_char,
    pub size: usize,
    pub len: usize,
}

#[macro_export]
macro_rules! DECLARE_SEQ_BUF {
    ($name:ident, $size:expr) => {
        let mut $name = $crate::seq_buf {
            buffer: {
                let mut storage = [0 as ::core::ffi::c_char; $size];
                storage.as_mut_ptr()
            },
            size: $size,
            len: 0,
        };
    };
}

pub unsafe fn seq_buf_clear(s: *mut seq_buf) {
    (*s).len = 0;
    if (*s).size != 0 {
        *(*s).buffer = 0;
    }
}

pub unsafe fn seq_buf_init(s: *mut seq_buf, buf: *mut ::core::ffi::c_char, size: ::core::ffi::c_uint) {
    (*s).buffer = buf;
    (*s).size = size as usize;
    seq_buf_clear(s);
}

/*
 * seq_buf have a buffer that might overflow. When this happens
 * len is set to be greater than size.
 */
pub unsafe fn seq_buf_has_overflowed(s: *mut seq_buf) -> bool {
    (*s).len > (*s).size
}

pub unsafe fn seq_buf_set_overflow(s: *mut seq_buf) {
    (*s).len = (*s).size.wrapping_add(1);
}

/* How much buffer is left on the seq_buf? */
pub unsafe fn seq_buf_buffer_left(s: *mut seq_buf) -> ::core::ffi::c_uint {
    if seq_buf_has_overflowed(s) {
        return 0;
    }
    ((*s).size - (*s).len) as ::core::ffi::c_uint
}

/* How much buffer was written? */
pub unsafe fn seq_buf_used(s: *mut seq_buf) -> ::core::ffi::c_uint {
    core::cmp::min((*s).len, (*s).size) as ::core::ffi::c_uint
}

pub unsafe fn seq_buf_str(s: *mut seq_buf) -> *const ::core::ffi::c_char {
    /* WARN_ON(s->size == 0) */
    if (*s).size == 0 {
        return b"\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if seq_buf_buffer_left(s) != 0 {
        *(*s).buffer.add((*s).len) = 0;
    } else {
        *(*s).buffer.add((*s).size - 1) = 0;
    }
    (*s).buffer
}

pub unsafe fn seq_buf_get_buf(s: *mut seq_buf, bufp: *mut *mut ::core::ffi::c_char) -> usize {
    /* WARN_ON(s->len > s->size + 1); */
    if (*s).len < (*s).size {
        *bufp = (*s).buffer.add((*s).len);
        return (*s).size - (*s).len;
    }
    *bufp = core::ptr::null_mut();
    0
}

pub unsafe fn seq_buf_commit(s: *mut seq_buf, num: ::core::ffi::c_int) {
    if num < 0 {
        seq_buf_set_overflow(s);
    } else {
        /* num must be negative on overflow */
        /* BUG_ON(s->len + num > s->size); */
        (*s).len += num as usize;
    }
}

pub unsafe fn seq_buf_pop(s: *mut seq_buf) -> ::core::ffi::c_int {
    if (*s).len == 0 {
        return -1;
    }
    (*s).len -= 1;
    *(*s).buffer.add((*s).len) as ::core::ffi::c_int
}

unsafe extern "C" {
    pub fn seq_buf_printf(s: *mut seq_buf, fmt: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    pub fn seq_buf_vprintf(s: *mut seq_buf, fmt: *const ::core::ffi::c_char, args: ::core::ffi::VaList<'_>) -> ::core::ffi::c_int;
    pub fn seq_buf_print_seq(m: *mut seq_file, s: *mut seq_buf) -> ::core::ffi::c_int;
    pub fn seq_buf_to_user(s: *mut seq_buf, ubuf: *mut ::core::ffi::c_char, start: usize, cnt: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn seq_buf_puts(s: *mut seq_buf, str_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn seq_buf_putc(s: *mut seq_buf, c: u8) -> ::core::ffi::c_int;
    pub fn seq_buf_putmem(s: *mut seq_buf, mem: *const ::core::ffi::c_void, len: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn seq_buf_putmem_hex(s: *mut seq_buf, mem: *const ::core::ffi::c_void, len: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn seq_buf_path(s: *mut seq_buf, path: *const path, esc: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn seq_buf_hex_dump(s: *mut seq_buf, prefix_str: *const ::core::ffi::c_char, prefix_type: ::core::ffi::c_int, rowsize: ::core::ffi::c_int, groupsize: ::core::ffi::c_int, buf: *const ::core::ffi::c_void, len: usize, ascii: bool) -> ::core::ffi::c_int;
    pub fn seq_buf_do_printk(s: *mut seq_buf, lvl: *const ::core::ffi::c_char);
}

#[cfg(CONFIG_BINARY_PRINTF)]
unsafe extern "C" {
    pub fn seq_buf_bprintf(s: *mut seq_buf, fmt: *const ::core::ffi::c_char, binary: *const u32) -> ::core::ffi::c_int;
}

/* External types supplied by the corresponding kernel headers. */
#[allow(non_camel_case_types)]
pub enum seq_file {}
#[allow(non_camel_case_types)]
pub enum path {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
