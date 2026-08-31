/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Rust translation of perf/util/strbuf.h.
 *
 * Original C dependencies:
 * - <assert.h>
 * - <stdarg.h>
 * - <stddef.h>
 * - <string.h>
 * - <linux/compiler.h>
 * - <sys/types.h>
 */

use std::ffi::c_void;
use std::os::raw::{c_char, c_int};

pub type ssize_t = isize;

unsafe extern "C" {
    pub static mut strbuf_slopbuf: [c_char; 0];
}

/*
 * Strbuf's can be use in many ways: as a byte array, or to store arbitrary
 * long, overflow safe strings.
 *
 * Strbufs has some invariants that are very important to keep in mind:
 *
 * 1. the ->buf member is always malloc-ed, hence strbuf's can be used to
 *    build complex strings/buffers whose final size isn't easily known.
 *
 *    It is NOT legal to copy the ->buf pointer away.
 *    `strbuf_detach' is the operation that detaches a buffer from its shell
 *    while keeping the shell valid wrt its invariants.
 *
 * 2. the ->buf member is a byte array that has at least ->len + 1 bytes
 *    allocated. The extra byte is used to store a '\0', allowing the ->buf
 *    member to be a valid C-string. Every strbuf function ensure this
 *    invariant is preserved.
 *
 *    Note that it is OK to "play" with the buffer directly if you work it
 *    that way:
 *
 *    strbuf_grow(sb, SOME_SIZE);
 *       ... Here, the memory array starting at sb->buf, and of length
 *       ... strbuf_avail(sb) is all yours, and you are sure that
 *       ... strbuf_avail(sb) is at least SOME_SIZE.
 *    strbuf_setlen(sb, sb->len + SOME_OTHER_SIZE);
 *
 *    Of course, SOME_OTHER_SIZE must be smaller or equal to strbuf_avail(sb).
 *
 *    Doing so is safe, though if it has to be done in many places, adding the
 *    missing API to the strbuf module is the way to go.
 *
 *    XXX: do _not_ assume that the area that is yours is of size ->alloc - 1
 *         even if it's true in the current implementation. Alloc is somehow a
 *         "private" member that should not be messed with.
 */
#[repr(C)]
pub struct strbuf {
    pub alloc: usize,
    pub len: usize,
    pub buf: *mut c_char,
}

#[macro_export]
macro_rules! STRBUF_INIT {
    () => {
        $crate::strbuf {
            alloc: 0,
            len: 0,
            buf: unsafe { $crate::strbuf_slopbuf.as_mut_ptr() },
        }
    };
}

unsafe extern "C" {
    /*----- strbuf life cycle -----*/
    pub fn strbuf_init(buf: *mut strbuf, hint: ssize_t) -> c_int;
    pub fn strbuf_release(buf: *mut strbuf);
    pub fn strbuf_detach(buf: *mut strbuf, len: *mut usize) -> *mut c_char;

    pub fn strbuf_grow(buf: *mut strbuf, size: usize) -> c_int;

    /*----- add data in your buffer -----*/
    pub fn strbuf_addch(sb: *mut strbuf, c: c_int) -> c_int;

    pub fn strbuf_add(buf: *mut strbuf, data: *const c_void, len: usize) -> c_int;

    /*
     * Original C declaration used __printf(2, 3). Rust has no direct source-level
     * equivalent for that compiler format-checking annotation here.
     */
    pub fn strbuf_addf(sb: *mut strbuf, fmt: *const c_char, ...) -> c_int;

    /* XXX: if read fails, any partial read is undone */
    pub fn strbuf_read(buf: *mut strbuf, fd: c_int, hint: ssize_t) -> ssize_t;
}

/*----- strbuf size related -----*/
#[inline]
pub unsafe fn strbuf_avail(sb: *const strbuf) -> ssize_t {
    if unsafe { (*sb).alloc } != 0 {
        unsafe { ((*sb).alloc - (*sb).len - 1) as ssize_t }
    } else {
        0
    }
}

#[inline]
pub unsafe fn strbuf_setlen(sb: *mut strbuf, len: usize) -> c_int {
    if unsafe { (*sb).alloc } == 0 {
        let ret = unsafe { strbuf_grow(sb, 0) };
        if ret != 0 {
            return ret;
        }
    }
    assert!(len < unsafe { (*sb).alloc });
    unsafe {
        (*sb).len = len;
        *(*sb).buf.add(len) = b'\0' as c_char;
    }
    0
}

#[inline]
pub unsafe fn strbuf_addstr(sb: *mut strbuf, s: *const c_char) -> c_int {
    unsafe { strbuf_add(sb, s as *const c_void, strlen(s)) }
}

unsafe extern "C" {
    pub fn strlen(s: *const c_char) -> usize;
}
