/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Lightweight buffered reading library.
 *
 * Copyright 2019 Google LLC.
 */

use core::ffi::{c_char, c_int, c_short, c_void};

pub type ssize_t = isize;
pub type size_t = usize;
pub type __u64 = u64;

/* Constants supplied by errno.h and poll.h in the C source. */
pub const POLLIN: c_short = 0x0001;
pub const ETIMEDOUT: c_int = 110;
pub const EIO: c_int = 5;
pub const ENOMEM: c_int = 12;

#[repr(C)]
pub struct pollfd {
    pub fd: c_int,
    pub events: c_short,
    pub revents: c_short,
}

unsafe extern "C" {
    pub static mut errno: c_int;

    pub fn poll(fds: *mut pollfd, nfds: usize, timeout: c_int) -> c_int;
    pub fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    pub fn free(ptr: *mut c_void);
    pub fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    pub fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
}

#[repr(C)]
pub struct io {
    /* File descriptor being read/ */
    pub fd: c_int,
    /* Size of the read buffer. */
    pub buf_len: u32,
    /* Pointer to storage for buffering read. */
    pub buf: *mut c_char,
    /* End of the storage. */
    pub end: *mut c_char,
    /* Currently accessed data pointer. */
    pub data: *mut c_char,
    /* Read timeout, 0 implies no timeout. */
    pub timeout_ms: c_int,
    /* Set true on when the end of file on read error. */
    pub eof: bool,
}

pub unsafe fn io__init(io: *mut io, fd: c_int, buf: *mut c_char, buf_len: u32) {
    unsafe {
        (*io).fd = fd;
        (*io).buf_len = buf_len;
        (*io).buf = buf;
        (*io).end = buf;
        (*io).data = buf;
        (*io).timeout_ms = 0;
        (*io).eof = false;
    }
}

/* Read from fd filling the buffer. Called when io->data == io->end. */
pub unsafe fn io__fill_buffer(io: *mut io) -> c_int {
    let mut n: ssize_t;

    unsafe {
        if (*io).eof {
            return -1;
        }

        if (*io).timeout_ms != 0 {
            let mut pfds = [pollfd {
                fd: (*io).fd,
                events: POLLIN,
                revents: 0,
            }];

            n = poll(pfds.as_mut_ptr(), 1, (*io).timeout_ms) as ssize_t;
            if n == 0 {
                errno = ETIMEDOUT;
            }
            if n > 0 && (pfds[0].revents & POLLIN) == 0 {
                errno = EIO;
                n = -1;
            }
            if n <= 0 {
                (*io).eof = true;
                return -1;
            }
        }
        n = read((*io).fd, (*io).buf as *mut c_void, (*io).buf_len as size_t);

        if n <= 0 {
            (*io).eof = true;
            return -1;
        }
        (*io).data = (*io).buf.add(0);
        (*io).end = (*io).buf.add(n as usize);
        0
    }
}

/* Reads one character from the "io" file with similar semantics to fgetc. */
pub unsafe fn io__get_char(io: *mut io) -> c_int {
    unsafe {
        if (*io).data == (*io).end {
            let ret = io__fill_buffer(io);

            if ret != 0 {
                return ret;
            }
        }
        let ch = *(*io).data as c_int;
        (*io).data = (*io).data.add(1);
        ch
    }
}

/* Read a hexadecimal value with no 0x prefix into the out argument hex. If the
 * first character isn't hexadecimal returns -2, io->eof returns -1, otherwise
 * returns the character after the hexadecimal value which may be -1 for eof.
 * If the read value is larger than a u64 the high-order bits will be dropped.
 */
pub unsafe fn io__get_hex(io: *mut io, hex: *mut __u64) -> c_int {
    let mut first_read = true;

    unsafe {
        *hex = 0;
        loop {
            let ch = io__get_char(io);

            if ch < 0 {
                return ch;
            }
            if ch >= b'0' as c_int && ch <= b'9' as c_int {
                *hex = (*hex << 4) | (ch - b'0' as c_int) as __u64;
            } else if ch >= b'a' as c_int && ch <= b'f' as c_int {
                *hex = (*hex << 4) | (ch - b'a' as c_int + 10) as __u64;
            } else if ch >= b'A' as c_int && ch <= b'F' as c_int {
                *hex = (*hex << 4) | (ch - b'A' as c_int + 10) as __u64;
            } else if first_read {
                return -2;
            } else {
                return ch;
            }
            first_read = false;
        }
    }
}

/* Read a positive decimal value with out argument dec. If the first character
 * isn't a decimal returns -2, io->eof returns -1, otherwise returns the
 * character after the decimal value which may be -1 for eof. If the read value
 * is larger than a u64 the high-order bits will be dropped.
 */
pub unsafe fn io__get_dec(io: *mut io, dec: *mut __u64) -> c_int {
    let mut first_read = true;

    unsafe {
        *dec = 0;
        loop {
            let ch = io__get_char(io);

            if ch < 0 {
                return ch;
            }
            if ch >= b'0' as c_int && ch <= b'9' as c_int {
                *dec = (*dec).wrapping_mul(10).wrapping_add((ch - b'0' as c_int) as __u64);
            } else if first_read {
                return -2;
            } else {
                return ch;
            }
            first_read = false;
        }
    }
}

/* Read up to and including the first delim. */
pub unsafe fn io__getdelim(
    io: *mut io,
    line_out: *mut *mut c_char,
    line_len_out: *mut size_t,
    delim: c_int,
) -> ssize_t {
    let mut buf = [0 as c_char; 128];
    let mut buf_pos: c_int = 0;
    let mut line: *mut c_char = core::ptr::null_mut();
    let mut temp: *mut c_char;
    let mut line_len: size_t = 0;
    let mut ch: c_int = 0;

    unsafe {
        /* TODO: reuse previously allocated memory. */
        free(*line_out as *mut c_void);
        while ch != delim {
            ch = io__get_char(io);

            if ch < 0 {
                break;
            }

            if buf_pos as size_t == core::mem::size_of_val(&buf) {
                temp = realloc(
                    line as *mut c_void,
                    line_len + core::mem::size_of_val(&buf),
                ) as *mut c_char;
                if temp.is_null() {
                    free(line as *mut c_void);
                    *line_out = core::ptr::null_mut();
                    *line_len_out = 0;
                    return -ENOMEM as ssize_t;
                }
                line = temp;
                memcpy(
                    line.add(line_len) as *mut c_void,
                    buf.as_ptr() as *const c_void,
                    core::mem::size_of_val(&buf),
                );
                line_len += core::mem::size_of_val(&buf);
                buf_pos = 0;
            }
            buf[buf_pos as usize] = ch as c_char;
            buf_pos += 1;
        }
        temp = realloc(line as *mut c_void, line_len + buf_pos as size_t + 1) as *mut c_char;
        if temp.is_null() {
            free(line as *mut c_void);
            *line_out = core::ptr::null_mut();
            *line_len_out = 0;
            return -ENOMEM as ssize_t;
        }
        line = temp;
        memcpy(
            line.add(line_len) as *mut c_void,
            buf.as_ptr() as *const c_void,
            buf_pos as size_t,
        );
        *line.add(line_len + buf_pos as size_t) = b'\0' as c_char;
        line_len += buf_pos as size_t;
        *line_out = line;
        *line_len_out = line_len;
        line_len as ssize_t
    }
}

pub unsafe fn io__getline(
    io: *mut io,
    line_out: *mut *mut c_char,
    line_len_out: *mut size_t,
) -> ssize_t {
    unsafe { io__getdelim(io, line_out, line_len_out, /*delim=*/ b'\n' as c_int) }
}
