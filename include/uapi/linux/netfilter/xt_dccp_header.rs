/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translated from the Linux UAPI header <linux/netfilter/xt_dccp.h>.

pub const XT_DCCP_SRC_PORTS: u32 = 0x01;
pub const XT_DCCP_DEST_PORTS: u32 = 0x02;
pub const XT_DCCP_TYPE: u32 = 0x04;
pub const XT_DCCP_OPTION: u32 = 0x08;

pub const XT_DCCP_VALID_FLAGS: u32 = 0x0f;

#[repr(C)]
pub struct xt_dccp_info {
    pub dpts: [u16; 2], /* Min, Max */
    pub spts: [u16; 2], /* Min, Max */

    pub flags: u16,
    pub invflags: u16,

    pub typemask: u16,
    pub option: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
