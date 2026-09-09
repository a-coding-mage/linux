/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency: `__u16` is supplied by the Linux types bindings.

#[repr(C)]
pub struct xt_tcpmss_info {
    pub mss: __u16,
}

pub const XT_TCPMSS_CLAMP_PMTU: __u16 = 0xffff;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
