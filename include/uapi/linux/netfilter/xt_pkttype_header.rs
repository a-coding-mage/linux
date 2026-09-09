/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
// Translated from the C header guard: _XT_PKTTYPE_H.

#[repr(C)]
pub struct xt_pkttype_info {
    pub pkttype: std::os::raw::c_int,
    pub invert: std::os::raw::c_int,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
