/* SPDX-License-Identifier: GPL-2.0 */

// Source dependencies:
// <linux/netfilter.h>
// <linux/skbuff.h>
// <linux/types.h>
// <net/netfilter/nf_conntrack_expect.h>

#[repr(C)]
pub struct tftphdr {
    pub opcode: __be16,
}

pub const TFTP_OPCODE_READ: u32 = 1;
pub const TFTP_OPCODE_WRITE: u32 = 2;
pub const TFTP_OPCODE_DATA: u32 = 3;
pub const TFTP_OPCODE_ACK: u32 = 4;
pub const TFTP_OPCODE_ERROR: u32 = 5;

pub type nf_nat_tftp_hook_fn = unsafe extern "C" fn(
    skb: *mut sk_buff,
    ct: *mut nf_conn,
    ctinfo: ip_conntrack_info,
    exp: *mut nf_conntrack_expect,
) -> core::ffi::c_uint;

extern "C" {
    pub static mut nf_nat_tftp_hook: *mut nf_nat_tftp_hook_fn;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
