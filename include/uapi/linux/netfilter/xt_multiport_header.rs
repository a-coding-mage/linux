/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency: __u8 and __u16 from <linux/types.h> are represented by u8 and u16.

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum xt_multiport_flags {
    XT_MULTIPORT_SOURCE,
    XT_MULTIPORT_DESTINATION,
    XT_MULTIPORT_EITHER,
}

pub const XT_MULTI_PORTS: usize = 15;

/// Must fit inside union xt_matchinfo: 16 bytes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_multiport {
    pub flags: u8,                 // Type of comparison
    pub count: u8,                 // Number of ports
    pub ports: [u16; XT_MULTI_PORTS], // Ports
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_multiport_v1 {
    pub flags: u8,                 // Type of comparison
    pub count: u8,                 // Number of ports
    pub ports: [u16; XT_MULTI_PORTS], // Ports
    pub pflags: [u8; XT_MULTI_PORTS], // Port flags
    pub invert: u8,                // Invert flag
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
