/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Header guard: _UAPI_XT_CONNLABEL_H
// Dependency: __u16 is supplied by <linux/types.h>.

pub const XT_CONNLABEL_MAXBIT: i32 = 127;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum xt_connlabel_mtopts {
    XT_CONNLABEL_OP_INVERT = 1 << 0,
    XT_CONNLABEL_OP_SET = 1 << 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_connlabel_mtinfo {
    pub bit: u16,
    pub options: u16,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
