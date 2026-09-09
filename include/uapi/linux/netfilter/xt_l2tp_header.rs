/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency supplied by <linux/types.h> in the original header.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum xt_l2tp_type {
    XT_L2TP_TYPE_CONTROL,
    XT_L2TP_TYPE_DATA,
}

/* L2TP matching stuff */
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct xt_l2tp_info {
    pub tid: __u32,       /* tunnel id */
    pub sid: __u32,       /* session id */
    pub version: __u8,    /* L2TP protocol version */
    pub type_: __u8,      /* L2TP packet type */
    pub flags: __u8,      /* which fields to match */
}

pub const XT_L2TP_TID: i32 = 1 << 0;      /* match L2TP tunnel id */
pub const XT_L2TP_SID: i32 = 1 << 1;      /* match L2TP session id */
pub const XT_L2TP_VERSION: i32 = 1 << 2;  /* match L2TP protocol version */
pub const XT_L2TP_TYPE: i32 = 1 << 3;     /* match L2TP packet type */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
