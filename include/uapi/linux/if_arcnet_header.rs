/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * Global definitions for the ARCnet interface.
 *
 * Translated from the corresponding Linux UAPI C header.
 */

// Dependencies supplied by the surrounding UAPI bindings:
// linux/types.h and linux/if_ether.h

/* These are the defined ARCnet Protocol IDs. */

/* CAP mode: no macro, but uses 1-8. */

/* RFC1201 Protocol IDs */
pub const ARC_P_IP: u8 = 212; // 0xD4
pub const ARC_P_IPV6: u8 = 196; // 0xC4: RFC2497
pub const ARC_P_ARP: u8 = 213; // 0xD5
pub const ARC_P_RARP: u8 = 214; // 0xD6
pub const ARC_P_IPX: u8 = 250; // 0xFA
pub const ARC_P_NOVELL_EC: u8 = 236; // 0xEC

/* Old RFC1051 Protocol IDs */
pub const ARC_P_IP_RFC1051: u8 = 240; // 0xF0
pub const ARC_P_ARP_RFC1051: u8 = 241; // 0xF1

/* MS LanMan/WfWg "NDIS" encapsulation */
pub const ARC_P_ETHER: u8 = 232; // 0xE8

/* Unsupported/indirectly supported protocols */
pub const ARC_P_DATAPOINT_BOOT: u8 = 0; // very old Datapoint equipment
pub const ARC_P_DATAPOINT_MOUNT: u8 = 1;
pub const ARC_P_POWERLAN_BEACON: u8 = 8; // Probably ATA-Netbios related
pub const ARC_P_POWERLAN_BEACON2: u8 = 243; // 0xF3
pub const ARC_P_LANSOFT: u8 = 251; // 0xFB - what is this?
pub const ARC_P_ATALK: u8 = 0xDD;

/* Hardware address length */
pub const ARCNET_ALEN: usize = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arc_rfc1201 {
    pub proto: u8,
    pub split_flag: u8,
    pub sequence: u16, // __be16
    pub payload: [u8; 0],
}
pub const RFC1201_HDR_SIZE: usize = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arc_rfc1051 {
    pub proto: u8,
    pub payload: [u8; 0],
}
pub const RFC1051_HDR_SIZE: usize = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arc_eth_encap {
    pub proto: u8,
    pub eth: core::mem::ManuallyDrop<ethhdr>,
    pub payload: [u8; 0],
}
pub const ETH_ENCAP_HDR_SIZE: usize = 14;

#[repr(C)]
#[derive(Copy, Clone)]
pub union arc_cap_mes {
    pub ack: u8,
    pub raw: [u8; 0], // 507 bytes
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arc_cap {
    pub proto: u8,
    pub cookie: [u8; core::mem::size_of::<i32>()],
    /* Actually NOT sent over the network */
    pub mes: arc_cap_mes,
}

/* The data needed by the actual arcnet hardware. */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arc_hardware {
    pub source: u8, // source ARCnet - filled in automagically
    pub dest: u8, // destination ARCnet - 0 for broadcast
    pub offset: [u8; 2], // offset bytes (some weird semantics)
}
pub const ARC_HDR_SIZE: usize = 4;

/* This is an ARCnet frame header, as seen by the kernel and userspace. */
#[repr(C)]
#[derive(Copy, Clone)]
pub union archdr_soft {
    pub rfc1201: arc_rfc1201,
    pub rfc1051: arc_rfc1051,
    pub eth_encap: arc_eth_encap,
    pub cap: arc_cap,
    pub raw: [u8; 0], // 508 bytes
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct archdr {
    /* hardware requirements */
    pub hard: arc_hardware,
    /* arcnet encapsulation-specific bits */
    pub soft: archdr_soft,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
