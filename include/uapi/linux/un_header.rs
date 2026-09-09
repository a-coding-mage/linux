/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translated from the Linux UAPI header <linux/un.h>.
// The declarations referenced from <linux/socket.h> are supplied externally.

pub const UNIX_PATH_MAX: usize = 108;

#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: __kernel_sa_family_t, // AF_UNIX
    pub sun_path: [::core::ffi::c_char; UNIX_PATH_MAX], // pathname
}

pub const SIOCUNIXFILE: u32 = SIOCPROTOPRIVATE + 0; // open a socket file with O_PATH

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
