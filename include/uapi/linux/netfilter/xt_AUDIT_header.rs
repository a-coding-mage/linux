/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Header file for iptables xt_AUDIT target
 *
 * (C) 2010-2011 Thomas Graf <tgraf@redhat.com>
 * (C) 2010-2011 Red Hat, Inc.
 */

// Dependency equivalent of <linux/types.h> is supplied externally.

pub const XT_AUDIT_TYPE_ACCEPT: i32 = 0;
pub const XT_AUDIT_TYPE_DROP: i32 = 1;
pub const XT_AUDIT_TYPE_REJECT: i32 = 2;
pub const __XT_AUDIT_TYPE_MAX: i32 = 3;

pub const XT_AUDIT_TYPE_MAX: i32 = __XT_AUDIT_TYPE_MAX - 1;

#[repr(C)]
pub struct xt_audit_info {
    pub type_: u8, /* XT_AUDIT_TYPE_* */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
