/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * ioctl definitions for qeth driver
 *
 * Copyright IBM Corp. 2004
 *
 * Author(s): Thomas Spatzier <tspat@de.ibm.com>
 *
 */

/* C dependencies: linux/types.h and linux/ioctl.h. */

pub const SIOC_QETH_ARP_SET_NO_ENTRIES: u32 = SIOCDEVPRIVATE;
pub const SIOC_QETH_ARP_QUERY_INFO: u32 = SIOCDEVPRIVATE + 1;
pub const SIOC_QETH_ARP_ADD_ENTRY: u32 = SIOCDEVPRIVATE + 2;
pub const SIOC_QETH_ARP_REMOVE_ENTRY: u32 = SIOCDEVPRIVATE + 3;
pub const SIOC_QETH_ARP_FLUSH_CACHE: u32 = SIOCDEVPRIVATE + 4;
pub const SIOC_QETH_ADP_SET_SNMP_CONTROL: u32 = SIOCDEVPRIVATE + 5;
pub const SIOC_QETH_GET_CARD_TYPE: u32 = SIOCDEVPRIVATE + 6;
pub const SIOC_QETH_QUERY_OAT: u32 = SIOCDEVPRIVATE + 7;

#[repr(C, packed)]
pub struct qeth_arp_cache_entry {
    pub macaddr: [u8; 6],
    pub reserved1: [u8; 2],
    pub ipaddr: [u8; 16], /* for both IPv4 and IPv6 */
    pub reserved2: [u8; 32],
}

#[repr(i32)]
pub enum qeth_arp_ipaddrtype {
    QETHARP_IP_ADDR_V4 = 1,
    QETHARP_IP_ADDR_V6 = 2,
}

#[repr(C, packed)]
pub struct qeth_arp_entrytype {
    pub mac: u8,
    pub ip: u8,
}

pub const QETH_QARP_MEDIASPECIFIC_BYTES: usize = 32;
pub const QETH_QARP_MACADDRTYPE_BYTES: usize = 1;

#[repr(C, packed)]
pub struct qeth_arp_qi_entry7 {
    pub media_specific: [u8; QETH_QARP_MEDIASPECIFIC_BYTES],
    pub r#type: qeth_arp_entrytype,
    pub macaddr: [u8; 6],
    pub ipaddr: [u8; 4],
}

#[repr(C, packed)]
pub struct qeth_arp_qi_entry7_ipv6 {
    pub media_specific: [u8; QETH_QARP_MEDIASPECIFIC_BYTES],
    pub r#type: qeth_arp_entrytype,
    pub macaddr: [u8; 6],
    pub ipaddr: [u8; 16],
}

#[repr(C, packed)]
pub struct qeth_arp_qi_entry7_short {
    pub r#type: qeth_arp_entrytype,
    pub macaddr: [u8; 6],
    pub ipaddr: [u8; 4],
}

#[repr(C, packed)]
pub struct qeth_arp_qi_entry7_short_ipv6 {
    pub r#type: qeth_arp_entrytype,
    pub macaddr: [u8; 6],
    pub ipaddr: [u8; 16],
}

#[repr(C, packed)]
pub struct qeth_arp_qi_entry5 {
    pub media_specific: [u8; QETH_QARP_MEDIASPECIFIC_BYTES],
    pub r#type: qeth_arp_entrytype,
    pub ipaddr: [u8; 4],
}

#[repr(C, packed)]
pub struct qeth_arp_qi_entry5_ipv6 {
    pub media_specific: [u8; QETH_QARP_MEDIASPECIFIC_BYTES],
    pub r#type: qeth_arp_entrytype,
    pub ipaddr: [u8; 16],
}

#[repr(C, packed)]
pub struct qeth_arp_qi_entry5_short {
    pub r#type: qeth_arp_entrytype,
    pub ipaddr: [u8; 4],
}

#[repr(C, packed)]
pub struct qeth_arp_qi_entry5_short_ipv6 {
    pub r#type: qeth_arp_entrytype,
    pub ipaddr: [u8; 16],
}

/*
 * can be set by user if no "media specific information" is wanted
 * -> saves a lot of space in user space buffer
 */
pub const QETH_QARP_STRIP_ENTRIES: u16 = 0x8000;
pub const QETH_QARP_WITH_IPV6: u16 = 0x4000;
pub const QETH_QARP_REQUEST_MASK: u16 = 0x00ff;

/* data sent to user space as result of query arp ioctl */
pub const QETH_QARP_USER_DATA_SIZE: usize = 20000;
pub const QETH_QARP_MASK_OFFSET: usize = 4;
pub const QETH_QARP_ENTRIES_OFFSET: usize = 6;

#[repr(C, packed)]
pub union qeth_arp_query_user_data_u {
    pub data_len: u32, /* set by user space program */
    pub no_entries: u32, /* set by kernel */
}

#[repr(C, packed)]
pub struct qeth_arp_query_user_data {
    pub u: qeth_arp_query_user_data_u,
    pub mask_bits: u16,
    pub entries: *mut core::ffi::c_char,
}

#[repr(C)]
pub struct qeth_query_oat_data {
    pub command: u32,
    pub buffer_len: u32,
    pub response_len: u32,
    pub ptr: u64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
