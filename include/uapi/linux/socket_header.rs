/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * Desired design of maximum size and alignment (see RFC2553)
 */
pub const _K_SS_MAXSIZE: usize = 128; /* Implementation specific max size */

pub type __kernel_sa_family_t = u16;

/*
 * The definition uses anonymous union and struct in order to control the
 * default alignment.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __kernel_sockaddr_storage_data {
    pub ss_family: __kernel_sa_family_t, /* address family */
    /* Following field(s) are implementation specific */
    pub __data: [core::ffi::c_char; _K_SS_MAXSIZE - core::mem::size_of::<u16>()],
    /* space to achieve desired size, */
    /* _SS_MAXSIZE value minus size of ss_family */
}

#[repr(C)]
pub union __kernel_sockaddr_storage_union {
    pub data: __kernel_sockaddr_storage_data,
    pub __align: *mut core::ffi::c_void, /* implementation specific desired alignment */
}

#[repr(C)]
pub struct __kernel_sockaddr_storage {
    pub __data_union: __kernel_sockaddr_storage_union,
}

pub const SOCK_SNDBUF_LOCK: i32 = 1;
pub const SOCK_RCVBUF_LOCK: i32 = 2;

pub const SOCK_BUF_LOCK_MASK: i32 = SOCK_SNDBUF_LOCK | SOCK_RCVBUF_LOCK;

pub const SOCK_TXREHASH_DEFAULT: i32 = 255;
pub const SOCK_TXREHASH_DISABLED: i32 = 0;
pub const SOCK_TXREHASH_ENABLED: i32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
