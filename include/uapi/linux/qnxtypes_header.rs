/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 *  Name                         : qnxtypes.h
 *  Author                       : Richard Frowijn
 *  Function                     : standard qnx types
 *  History                      : 22-03-1998 created
 *
 */

// Dependency supplied by linux/types.h in the C source.

pub type qnx4_nxtnt_t = __le16;
pub type qnx4_ftype_t = __u8;

#[repr(C)]
pub struct qnx4_xtnt_t {
    pub xtnt_blk: __le32,
    pub xtnt_size: __le32,
}

pub type qnx4_mode_t = __le16;
pub type qnx4_muid_t = __le16;
pub type qnx4_mgid_t = __le16;
pub type qnx4_off_t = __le32;
pub type qnx4_nlink_t = __le16;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
