/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translated from <linux/types.h>; __u16, __u8, and __be16 are represented by
// their corresponding fixed-width Rust integer types.

pub const EBT_VLAN_ID: u8 = 0x01;
pub const EBT_VLAN_PRIO: u8 = 0x02;
pub const EBT_VLAN_ENCAP: u8 = 0x04;
pub const EBT_VLAN_MASK: u8 = EBT_VLAN_ID | EBT_VLAN_PRIO | EBT_VLAN_ENCAP;
pub const EBT_VLAN_MATCH: &str = "vlan";

#[repr(C)]
pub struct ebt_vlan_info {
    pub id: u16,      /* VLAN ID {1-4095} */
    pub prio: u8,     /* VLAN User Priority {0-7} */
    pub encap: u16,   /* VLAN Encapsulated frame code {0-65535} */
    pub bitmask: u8,  /* Args bitmask bit 1=1 - ID arg,
                         bit 2=1 User-Priority arg, bit 3=1 encap */
    pub invflags: u8, /* Inverse bitmask  bit 1=1 - inversed ID arg,
                         bit 2=1 - inversed Pirority arg */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
