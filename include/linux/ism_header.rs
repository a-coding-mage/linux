/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  Internal Shared Memory
 *
 *  Definitions for the ISM module
 *
 *  Copyright IBM Corp. 2022
 */

use core::ffi::{c_char, c_void};

/* Dependency supplied by the surrounding kernel translation. */
/* #include <linux/workqueue.h> */

/* Unless we gain unexpected popularity, this limit should hold for a while */
pub const MAX_CLIENTS: usize = 8;
pub const ISM_NR_DMBS: usize = 1920;

#[repr(C)]
pub struct ism_dev {
    pub lock: spinlock_t, /* protects the ism device */
    pub cmd_lock: spinlock_t, /* serializes cmds */
    pub list: list_head,
    pub dibs: *mut dibs_dev,
    pub pdev: *mut pci_dev,

    pub sba: *mut ism_sba,
    pub sba_dma_addr: dma_addr_t,
    pub sba_bitmap: [u64; (ISM_NR_DMBS + 63) / 64],
    pub r#priv: [*mut c_void; MAX_CLIENTS],

    pub ieq: *mut ism_eq,
    pub ieq_dma_addr: dma_addr_t,

    pub ieq_idx: i32,

    pub subs: [*mut ism_client; MAX_CLIENTS],
}

#[repr(C)]
pub struct ism_event {
    pub type_: u32,
    pub code: u32,
    pub tok: u64,
    pub time: u64,
    pub info: u64,
}

#[repr(C)]
pub struct ism_client {
    pub name: *const c_char,
    pub handle_event: Option<unsafe extern "C" fn(*mut ism_dev, *mut ism_event)>,
    /* Private area - don't touch! */
    pub id: u8,
}

extern "C" {
    pub fn ism_register_client(client: *mut ism_client) -> i32;
    pub fn ism_unregister_client(client: *mut ism_client) -> i32;
    pub fn ism_get_smcd_ops() -> *const smcd_ops;
}

#[inline]
pub unsafe fn ism_get_priv(dev: *mut ism_dev, client: *mut ism_client) -> *mut c_void {
    (*dev).r#priv[(*client).id as usize]
}

#[inline]
pub unsafe fn ism_set_priv(
    dev: *mut ism_dev,
    client: *mut ism_client,
    priv_: *mut c_void,
) {
    (*dev).r#priv[(*client).id as usize] = priv_;
}

/* Types supplied by the surrounding kernel translation. */
// spinlock_t, list_head, dibs_dev, pci_dev, ism_sba, ism_eq, dma_addr_t,
// and smcd_ops are supplied by the surrounding kernel translation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
