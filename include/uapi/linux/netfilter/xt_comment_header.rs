/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Original C header guard: _XT_COMMENT_H

pub const XT_MAX_COMMENT_LEN: usize = 256;

#[repr(C)]
pub struct xt_comment_info {
    pub comment: [core::ffi::c_char; XT_MAX_COMMENT_LEN],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
