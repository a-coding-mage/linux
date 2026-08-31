// SPDX-License-Identifier: GPL-2.0
// Translated from perf/util/strbuf.c.
// Original dependencies: cache.h, debug.h, strbuf.h, linux/kernel.h,
// linux/string.h, linux/zalloc.h, errno.h, stdio.h, stdlib.h, unistd.h.

use core::ffi::{c_char, c_int, c_void};

pub type size_t = usize;
pub type ssize_t = isize;

pub const E2BIG: c_int = 7;
pub const ENOMEM: c_int = 12;
pub const EINVAL: c_int = 22;

#[repr(C)]
pub struct strbuf {
    pub alloc: size_t,
    pub len: size_t,
    pub buf: *mut c_char,
}

// va_list is supplied by the C ABI/dependencies for the final build.
pub type va_list = *mut c_void;

unsafe extern "C" {
    fn alloc_nr(x: size_t) -> size_t;
    fn zfree(ptr: *mut *mut c_char);
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn vsnprintf(str_: *mut c_char, size: size_t, format: *const c_char, ap: va_list) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn strbuf_setlen(sb: *mut strbuf, len: size_t) -> c_int;
    fn strbuf_avail(sb: *const strbuf) -> size_t;
    fn pr_debug(fmt: *const c_char, ...);
}

/*
 * Used as the default ->buf value, so that people can always assume
 * buf is non NULL and ->buf is NUL terminated even for a freshly
 * initialized strbuf.
 */
#[unsafe(no_mangle)]
pub static mut strbuf_slopbuf: [c_char; 1] = [0; 1];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strbuf_init(sb: *mut strbuf, hint: ssize_t) -> c_int {
    unsafe {
        (*sb).len = 0;
        (*sb).alloc = (*sb).len;
        (*sb).buf = strbuf_slopbuf.as_mut_ptr();
        if hint != 0 {
            return strbuf_grow(sb, hint as size_t);
        }
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strbuf_release(sb: *mut strbuf) {
    unsafe {
        if (*sb).alloc != 0 {
            zfree(&mut (*sb).buf);
            strbuf_init(sb, 0);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strbuf_detach(sb: *mut strbuf, sz: *mut size_t) -> *mut c_char {
    unsafe {
        let res = if (*sb).alloc != 0 {
            (*sb).buf
        } else {
            core::ptr::null_mut()
        };
        if !sz.is_null() {
            *sz = (*sb).len;
        }
        strbuf_init(sb, 0);
        res
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strbuf_grow(sb: *mut strbuf, extra: size_t) -> c_int {
    unsafe {
        let mut buf: *mut c_char;
        let mut nr: size_t = (*sb).len.wrapping_add(extra).wrapping_add(1);

        if nr < (*sb).alloc {
            return 0;
        }

        if nr <= (*sb).len {
            return -E2BIG;
        }

        if alloc_nr((*sb).alloc) > nr {
            nr = alloc_nr((*sb).alloc);
        }

        /*
         * Note that sb->buf == strbuf_slopbuf if sb->alloc == 0, and it is
         * a static variable. Thus we have to avoid passing it to realloc.
         */
        buf = realloc(
            if (*sb).alloc != 0 {
                (*sb).buf as *mut c_void
            } else {
                core::ptr::null_mut()
            },
            nr.wrapping_mul(core::mem::size_of::<c_char>()),
        ) as *mut c_char;
        if buf.is_null() {
            return -ENOMEM;
        }

        (*sb).buf = buf;
        (*sb).alloc = nr;
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strbuf_addch(sb: *mut strbuf, c: c_int) -> c_int {
    unsafe {
        let ret = strbuf_grow(sb, 1);
        if ret != 0 {
            return ret;
        }

        *(*sb).buf.add((*sb).len) = c as c_char;
        (*sb).len = (*sb).len.wrapping_add(1);
        *(*sb).buf.add((*sb).len) = b'\0' as c_char;
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strbuf_add(sb: *mut strbuf, data: *const c_void, len: size_t) -> c_int {
    unsafe {
        let ret = strbuf_grow(sb, len);
        if ret != 0 {
            return ret;
        }

        memcpy((*sb).buf.add((*sb).len) as *mut c_void, data, len);
        strbuf_setlen(sb, (*sb).len.wrapping_add(len))
    }
}

unsafe fn va_copy(dst: *mut va_list, src: va_list) {
    unsafe {
        *dst = src;
    }
}

unsafe fn va_end(_ap: va_list) {}

unsafe extern "C" fn strbuf_addv(sb: *mut strbuf, fmt: *const c_char, ap: va_list) -> c_int {
    unsafe {
        let mut len: c_int;
        let mut ret: c_int;
        let mut ap_saved: va_list = core::ptr::null_mut();

        if strbuf_avail(sb) == 0 {
            ret = strbuf_grow(sb, 64);
            if ret != 0 {
                return ret;
            }
        }

        va_copy(&mut ap_saved, ap);
        len = vsnprintf(
            (*sb).buf.add((*sb).len),
            (*sb).alloc.wrapping_sub((*sb).len),
            fmt,
            ap,
        );
        if len < 0 {
            va_end(ap_saved);
            return len;
        }
        if (len as size_t) > strbuf_avail(sb) {
            ret = strbuf_grow(sb, len as size_t);
            if ret != 0 {
                va_end(ap_saved);
                return ret;
            }
            len = vsnprintf(
                (*sb).buf.add((*sb).len),
                (*sb).alloc.wrapping_sub((*sb).len),
                fmt,
                ap_saved,
            );
            if (len as size_t) > strbuf_avail(sb) {
                pr_debug(c"this should not happen, your vsnprintf is broken".as_ptr());
                va_end(ap_saved);
                return -EINVAL;
            }
        }
        va_end(ap_saved);
        strbuf_setlen(sb, (*sb).len.wrapping_add(len as size_t))
    }
}

/*
 * Rust has no stable direct equivalent for defining a C variadic function that
 * constructs a va_list. The original interface and body are preserved here as
 * the intended C ABI translation:
 *
 * int strbuf_addf(struct strbuf *sb, const char *fmt, ...)
 * {
 *     va_list ap;
 *     int ret;
 *
 *     va_start(ap, fmt);
 *     ret = strbuf_addv(sb, fmt, ap);
 *     va_end(ap);
 *     return ret;
 * }
 */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strbuf_read(sb: *mut strbuf, fd: c_int, hint: ssize_t) -> ssize_t {
    unsafe {
        let oldlen: size_t = (*sb).len;
        let oldalloc: size_t = (*sb).alloc;
        let mut ret: c_int;

        ret = strbuf_grow(sb, if hint != 0 { hint as size_t } else { 8192 });
        if ret != 0 {
            return ret as ssize_t;
        }

        loop {
            let cnt: ssize_t;

            cnt = read(
                fd,
                (*sb).buf.add((*sb).len) as *mut c_void,
                (*sb).alloc.wrapping_sub((*sb).len).wrapping_sub(1),
            );
            if cnt < 0 {
                if oldalloc == 0 {
                    strbuf_release(sb);
                } else {
                    strbuf_setlen(sb, oldlen);
                }
                return cnt;
            }
            if cnt == 0 {
                break;
            }
            (*sb).len = (*sb).len.wrapping_add(cnt as size_t);
            ret = strbuf_grow(sb, 8192);
            if ret != 0 {
                return ret as ssize_t;
            }
        }

        *(*sb).buf.add((*sb).len) = b'\0' as c_char;
        (*sb).len.wrapping_sub(oldlen) as ssize_t
    }
}
