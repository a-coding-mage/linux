/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency supplied externally: <linux/if_ether.h>

pub const NAT_ARP_BIT: u32 = 0x0000_0010;

#[repr(C)]
pub struct ebt_nat_info {
    pub mac: [u8; ETH_ALEN],
    /* EBT_ACCEPT, EBT_DROP, EBT_CONTINUE or EBT_RETURN */
    pub target: i32,
}

pub const EBT_SNAT_TARGET: &str = "snat";
pub const EBT_DNAT_TARGET: &str = "dnat";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
