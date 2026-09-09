/* SPDX-License-Identifier: GPL-2.0 */

// External dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub union tty_buffer_link {
    pub next: *mut tty_buffer,
    pub free: llist_node,
}

#[repr(C, align(8))]
pub struct tty_buffer {
    pub link: tty_buffer_link,
    pub used: ::core::ffi::c_uint,
    pub size: ::core::ffi::c_uint,
    pub commit: ::core::ffi::c_uint,
    // Lazy update on recv, can become less than "read"
    pub lookahead: ::core::ffi::c_uint,
    pub read: ::core::ffi::c_uint,
    pub flags: bool,
    // Data points here
    pub data: [u8; 0],
}

#[inline]
pub unsafe fn char_buf_ptr(b: *mut tty_buffer, ofs: ::core::ffi::c_uint) -> *mut u8 {
    (*b).data.as_mut_ptr().add(ofs as usize)
}

#[inline]
pub unsafe fn flag_buf_ptr(b: *mut tty_buffer, ofs: ::core::ffi::c_uint) -> *mut u8 {
    char_buf_ptr(b, ofs).add((*b).size as usize)
}

#[repr(C)]
pub struct tty_bufhead {
    pub head: *mut tty_buffer, // Queue head
    pub flip_wq: *mut workqueue_struct,
    pub work: work_struct,
    pub lock: mutex,
    pub priority: atomic_t,
    pub sentinel: tty_buffer,
    pub free: llist_head, // Free queue head
    pub mem_used: atomic_t, // In-use buffers excluding free list
    pub mem_limit: ::core::ffi::c_int,
    pub tail: *mut tty_buffer, // Active buffer
}

/*
 * When a break, frame error, or parity error happens, these codes are
 * stuffed into the flags buffer.
 */
pub const TTY_NORMAL: ::core::ffi::c_uint = 0;
pub const TTY_BREAK: ::core::ffi::c_uint = 1;
pub const TTY_FRAME: ::core::ffi::c_uint = 2;
pub const TTY_PARITY: ::core::ffi::c_uint = 3;
pub const TTY_OVERRUN: ::core::ffi::c_uint = 4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
