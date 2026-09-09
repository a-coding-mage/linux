/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * Berkeley style UIO structures - Alan Cox 1994.
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License as
 * published by the Free Software Foundation; either version
 * 2 of the License, or (at your option) any later version.
 */

use core::ffi::c_void;

#[repr(C)]
pub struct iovec {
    /* BSD uses caddr_t (1003.1g requires void *) */
    pub iov_base: *mut c_void,
    /* Must be size_t (1003.1g) */
    pub iov_len: usize,
}

#[repr(C)]
pub struct dmabuf_cmsg {
    /* offset into the dmabuf where the frag starts. */
    pub frag_offset: u64,
    /* size of the frag. */
    pub frag_size: u32,
    /* token representing this frag for DEVMEM_DONTNEED. */
    pub frag_token: u32,
    /* dmabuf id this frag belongs to. */
    pub dmabuf_id: u32,
    /* Currently unused. Reserved for future uses. */
    pub flags: u32,
}

#[repr(C)]
pub struct dmabuf_token {
    pub token_start: u32,
    pub token_count: u32,
}

/*
 * UIO_MAXIOV shall be at least 16 1003.1g (5.4.1.1)
 */
pub const UIO_FASTIOV: u32 = 8;
pub const UIO_MAXIOV: u32 = 1024;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
