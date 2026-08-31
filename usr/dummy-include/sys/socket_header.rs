/* SPDX-License-Identifier: GPL-2.0-only */

/* Depends on linux/socket.h for __kernel_sa_family_t. */

#[repr(C)]
pub struct sockaddr {
    pub sa_family: __kernel_sa_family_t, /* address family, AF_xxx */
    pub sa_data: [::std::os::raw::c_char; 14], /* 14 bytes of protocol address */
}
