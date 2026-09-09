/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency intent: `__u8` and `__u32` are supplied by <linux/types.h>.

/*
 * This is intended for use by various security subsystems (but not
 * at the same time).
 *
 * 'mode' refers to the specific security subsystem which the
 * packets are being marked for.
 */
pub const SECMARK_MODE_SEL: u8 = 0x01; // SELinux
pub const SECMARK_SECCTX_MAX: usize = 256;

#[repr(C)]
pub struct xt_secmark_target_info {
    pub mode: __u8,
    pub secid: __u32,
    pub secctx: [core::ffi::c_char; SECMARK_SECCTX_MAX],
}

#[repr(C)]
pub struct xt_secmark_target_info_v1 {
    pub mode: __u8,
    pub secctx: [core::ffi::c_char; SECMARK_SECCTX_MAX],
    pub secid: __u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
