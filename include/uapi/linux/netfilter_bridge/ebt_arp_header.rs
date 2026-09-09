/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Translated from <linux/types.h> and <linux/if_ether.h>. */

pub const EBT_ARP_OPCODE: u32 = 0x01;
pub const EBT_ARP_HTYPE: u32 = 0x02;
pub const EBT_ARP_PTYPE: u32 = 0x04;
pub const EBT_ARP_SRC_IP: u32 = 0x08;
pub const EBT_ARP_DST_IP: u32 = 0x10;
pub const EBT_ARP_SRC_MAC: u32 = 0x20;
pub const EBT_ARP_DST_MAC: u32 = 0x40;
pub const EBT_ARP_GRAT: u32 = 0x80;
pub const EBT_ARP_MASK: u32 = EBT_ARP_OPCODE
    | EBT_ARP_HTYPE
    | EBT_ARP_PTYPE
    | EBT_ARP_SRC_IP
    | EBT_ARP_DST_IP
    | EBT_ARP_SRC_MAC
    | EBT_ARP_DST_MAC
    | EBT_ARP_GRAT;
pub const EBT_ARP_MATCH: &str = "arp";

#[repr(C)]
pub struct ebt_arp_info {
    pub htype: u16,
    pub ptype: u16,
    pub opcode: u16,
    pub saddr: u32,
    pub smsk: u32,
    pub daddr: u32,
    pub dmsk: u32,
    pub smaddr: [u8; 6],
    pub smmsk: [u8; 6],
    pub dmaddr: [u8; 6],
    pub dmmsk: [u8; 6],
    pub bitmask: u8,
    pub invflags: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
