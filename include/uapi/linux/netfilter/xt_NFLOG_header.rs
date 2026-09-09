/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* _XT_NFLOG_TARGET */

pub const XT_NFLOG_DEFAULT_GROUP: u32 = 0x1;
pub const XT_NFLOG_DEFAULT_THRESHOLD: u32 = 0;

pub const XT_NFLOG_MASK: u32 = 0x1;

/* This flag indicates that 'len' field in xt_nflog_info is set */
pub const XT_NFLOG_F_COPY_LEN: u32 = 0x1;

#[repr(C)]
pub struct xt_nflog_info {
    /* 'len' will be used iff you set XT_NFLOG_F_COPY_LEN in flags */
    pub len: u32,
    pub group: u16,
    pub threshold: u16,
    pub flags: u16,
    pub pad: u16,
    pub prefix: [::core::ffi::c_char; 64],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
