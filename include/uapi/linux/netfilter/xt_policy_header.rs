/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translated from xt_policy.h. C header dependencies are represented by the
// corresponding primitive Rust integer types and local address layouts.

pub const XT_POLICY_MAX_ELEM: usize = 4;

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum xt_policy_flags {
    XT_POLICY_MATCH_IN = 0x1,
    XT_POLICY_MATCH_OUT = 0x2,
    XT_POLICY_MATCH_NONE = 0x4,
    XT_POLICY_MATCH_STRICT = 0x8,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum xt_policy_modes {
    XT_POLICY_MODE_TRANSPORT = 0,
    XT_POLICY_MODE_TUNNEL = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct xt_policy_spec {
    pub bits: u8,
}

impl xt_policy_spec {
    pub const SADDR: u8 = 1 << 0;
    pub const DADDR: u8 = 1 << 1;
    pub const PROTO: u8 = 1 << 2;
    pub const MODE: u8 = 1 << 3;
    pub const SPI: u8 = 1 << 4;
    pub const REQID: u8 = 1 << 5;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union xt_policy_addr {
    pub a4: [u8; 4],
    pub a6: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_policy_elem_addr {
    pub saddr: xt_policy_addr,
    pub smask: xt_policy_addr,
    pub daddr: xt_policy_addr,
    pub dmask: xt_policy_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_policy_elem {
    pub addr: xt_policy_elem_addr,
    pub spi: u32,
    pub reqid: u32,
    pub proto: u8,
    pub mode: u8,
    pub match_: xt_policy_spec,
    pub invert: xt_policy_spec,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_policy_info {
    pub pol: [xt_policy_elem; XT_POLICY_MAX_ELEM],
    pub flags: u16,
    pub len: u16,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
