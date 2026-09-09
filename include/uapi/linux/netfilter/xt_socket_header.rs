/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency supplied by the Linux types bindings.

pub const XT_SOCKET_TRANSPARENT: i32 = 1 << 0;
pub const XT_SOCKET_NOWILDCARD: i32 = 1 << 1;
pub const XT_SOCKET_RESTORESKMARK: i32 = 1 << 2;

#[repr(C)]
pub struct xt_socket_mtinfo1 {
    pub flags: u8,
}

pub const XT_SOCKET_FLAGS_V1: i32 = XT_SOCKET_TRANSPARENT;

#[repr(C)]
pub struct xt_socket_mtinfo2 {
    pub flags: u8,
}

pub const XT_SOCKET_FLAGS_V2: i32 = XT_SOCKET_TRANSPARENT | XT_SOCKET_NOWILDCARD;

#[repr(C)]
pub struct xt_socket_mtinfo3 {
    pub flags: u8,
}

pub const XT_SOCKET_FLAGS_V3: i32 = XT_SOCKET_TRANSPARENT
    | XT_SOCKET_NOWILDCARD
    | XT_SOCKET_RESTORESKMARK;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
