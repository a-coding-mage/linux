/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency supplied by the corresponding Linux types definitions.

#[repr(C)]
pub enum xt_statistic_mode {
    XT_STATISTIC_MODE_RANDOM,
    XT_STATISTIC_MODE_NTH,
    __XT_STATISTIC_MODE_MAX,
}

pub const XT_STATISTIC_MODE_MAX: i32 = __XT_STATISTIC_MODE_MAX as i32 - 1;

#[repr(C)]
pub enum xt_statistic_flags {
    XT_STATISTIC_INVERT = 0x1,
}

pub const XT_STATISTIC_MASK: u32 = 0x1;

pub struct xt_statistic_priv;

#[repr(C)]
pub struct xt_statistic_info {
    pub mode: __u16,
    pub flags: __u16,
    pub u: xt_statistic_info_u,
    pub master: *mut xt_statistic_priv,
}

#[repr(C)]
pub union xt_statistic_info_u {
    pub random: xt_statistic_info_u_random,
    pub nth: xt_statistic_info_u_nth,
}

#[repr(C)]
pub struct xt_statistic_info_u_random {
    pub probability: __u32,
}

#[repr(C)]
pub struct xt_statistic_info_u_nth {
    pub every: __u32,
    pub packet: __u32,
    pub count: __u32, /* unused */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
