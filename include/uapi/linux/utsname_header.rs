/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
// C header guard: _UAPI_LINUX_UTSNAME_H

pub const __OLD_UTS_LEN: usize = 8;

#[repr(C)]
pub struct oldold_utsname {
    pub sysname: [i8; 9],
    pub nodename: [i8; 9],
    pub release: [i8; 9],
    pub version: [i8; 9],
    pub machine: [i8; 9],
}

pub const __NEW_UTS_LEN: usize = 64;

#[repr(C)]
pub struct old_utsname {
    pub sysname: [i8; 65],
    pub nodename: [i8; 65],
    pub release: [i8; 65],
    pub version: [i8; 65],
    pub machine: [i8; 65],
}

#[repr(C)]
pub struct new_utsname {
    pub sysname: [i8; __NEW_UTS_LEN + 1],
    pub nodename: [i8; __NEW_UTS_LEN + 1],
    pub release: [i8; __NEW_UTS_LEN + 1],
    pub version: [i8; __NEW_UTS_LEN + 1],
    pub machine: [i8; __NEW_UTS_LEN + 1],
    pub domainname: [i8; __NEW_UTS_LEN + 1],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
