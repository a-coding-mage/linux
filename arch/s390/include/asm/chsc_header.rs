/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright IBM Corp. 2020
 *
 * Author(s): Alexandra Winter <wintera@linux.ibm.com>
 *
 * Interface for Channel Subsystem Call
 */

// Dependency supplied by the surrounding translation unit:
// #include <uapi/asm/chsc.h>

/* struct from linux/notifier.h */
#[repr(C)]
pub struct notifier_block;

/**
 * Operation codes for CHSC PNSO:
 *    PNSO_OC_NET_BRIDGE_INFO - only addresses that are visible to a bridgeport
 *    PNSO_OC_NET_ADDR_INFO   - all addresses
 */
pub const PNSO_OC_NET_BRIDGE_INFO: u8 = 0;
pub const PNSO_OC_NET_ADDR_INFO: u8 = 3;

/**
 * struct chsc_pnso_naid_l2 - network address information descriptor
 * @nit:  Network interface token
 * @addr_lnid: network address and logical network id (VLAN ID)
 */
#[repr(C, packed)]
pub struct chsc_pnso_naid_l2 {
    pub nit: u64,
    pub addr_lnid: chsc_pnso_naid_l2_addr_lnid,
}

#[repr(C, packed)]
pub struct chsc_pnso_naid_l2_addr_lnid {
    pub mac: [u8; 6],
    pub lnid: u16,
}

#[repr(C, packed)]
pub struct chsc_pnso_resume_token {
    pub t1: u64,
    pub t2: u64,
}

#[repr(C, packed)]
pub struct chsc_pnso_naihdr {
    pub resume_token: chsc_pnso_resume_token,
    /* u32:32 */
    pub _bitfield0: u32,
    pub instance: u32,
    /* u32:24 */
    pub _bitfield1: [u8; 3],
    pub naids: u8,
    pub reserved: [u32; 3],
}

#[repr(C, packed)]
pub struct chsc_pnso_area {
    pub request: chsc_header,
    /* u8:2; u8 m:1; u8:5 */
    pub _flags0: u8,
    /* u8:2; u8 ssid:2; u8 fmt:4 */
    pub _flags1: u8,
    pub sch: u16,
    /* u8:8 */
    pub _flags2: u8,
    pub cssid: u8,
    /* u16:16 */
    pub _flags3: u16,
    pub oc: u8,
    /* u32:24 */
    pub _flags4: [u8; 3],
    pub resume_token: chsc_pnso_resume_token,
    /* u32 n:1; u32:31 */
    pub _flags5: u32,
    pub reserved: [u32; 3],
    pub response: chsc_header,
    /* u32:32 */
    pub _bitfield0: u32,
    pub naihdr: chsc_pnso_naihdr,
    pub entries: [chsc_pnso_naid_l2; 0],
}

/* __aligned(PAGE_SIZE) is retained as source intent; Rust alignment is supplied by the target ABI. */

/*
 * notifier interface - registered notifiers gets called on
 * the following events:
 * - ap config changed (CHSC_NOTIFY_AP_CFG)
 */
#[repr(i32)]
pub enum chsc_notify_type {
    CHSC_NOTIFY_AP_CFG = 3,
}

unsafe extern "C" {
    pub fn chsc_notifier_register(nb: *mut notifier_block) -> i32;
    pub fn chsc_notifier_unregister(nb: *mut notifier_block) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
