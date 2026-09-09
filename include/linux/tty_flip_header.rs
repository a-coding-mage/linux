/* SPDX-License-Identifier: GPL-2.0 */

// Declarations from linux/tty_buffer.h and linux/tty_port.h are supplied by
// other translation units.

use core::ffi::c_int;

#[repr(C)]
pub struct tty_ldisc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tty_buffer {
    pub flags: *mut u8,
    pub used: usize,
    pub size: usize,
}

#[repr(C)]
pub struct tty_buf_head {
    pub tail: *mut tty_buffer,
}

#[repr(C)]
pub struct tty_port {
    pub buf: tty_buf_head,
}

pub const TTY_NORMAL: u8 = 0;

unsafe extern "C" {
    pub fn tty_buffer_set_limit(port: *mut tty_port, limit: c_int) -> c_int;
    pub fn tty_buffer_space_avail(port: *mut tty_port) -> u32;
    pub fn tty_buffer_request_room(port: *mut tty_port, size: usize) -> c_int;
    pub fn __tty_insert_flip_string_flags(
        port: *mut tty_port,
        chars: *const u8,
        flags: *const u8,
        mutable_flags: bool,
        size: usize,
    ) -> usize;
    pub fn tty_prepare_flip_string(
        port: *mut tty_port,
        chars: *mut *mut u8,
        size: usize,
    ) -> usize;
    pub fn tty_flip_buffer_push(port: *mut tty_port);
    pub fn tty_buffer_lock_exclusive(port: *mut tty_port);
    pub fn tty_buffer_unlock_exclusive(port: *mut tty_port);
    pub fn tty_ldisc_receive_buf(
        ld: *mut tty_ldisc,
        p: *const u8,
        f: *const u8,
        count: usize,
    ) -> usize;
    pub fn flag_buf_ptr(tb: *mut tty_buffer, offset: usize) -> *mut u8;
    pub fn char_buf_ptr(tb: *mut tty_buffer, offset: usize) -> *mut u8;
}

#[inline]
pub unsafe fn tty_insert_flip_string_fixed_flag(
    port: *mut tty_port,
    chars: *const u8,
    flag: u8,
    size: usize,
) -> usize {
    unsafe { __tty_insert_flip_string_flags(port, chars, &flag, false, size) }
}

#[inline]
pub unsafe fn tty_insert_flip_string_flags(
    port: *mut tty_port,
    chars: *const u8,
    flags: *const u8,
    size: usize,
) -> usize {
    unsafe { __tty_insert_flip_string_flags(port, chars, flags, true, size) }
}

#[inline]
pub unsafe fn tty_insert_flip_char(port: *mut tty_port, ch: u8, flag: u8) -> usize {
    let tb = unsafe { (*(*port).buf.tail) };
    let change = tb.flags.is_null() && flag != TTY_NORMAL;
    if !change && tb.used < tb.size {
        if !tb.flags.is_null() {
            unsafe { *flag_buf_ptr((*port).buf.tail, tb.used) = flag };
        }
        unsafe { *char_buf_ptr((*port).buf.tail, tb.used) = ch };
        unsafe { (*(*port).buf.tail).used += 1 };
        return 1;
    }
    unsafe { __tty_insert_flip_string_flags(port, &ch, &flag, false, 1) }
}

#[inline]
pub unsafe fn tty_insert_flip_string(
    port: *mut tty_port,
    chars: *const u8,
    size: usize,
) -> usize {
    unsafe { tty_insert_flip_string_fixed_flag(port, chars, TTY_NORMAL, size) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
