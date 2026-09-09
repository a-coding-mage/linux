/* SPDX-License-Identifier: GPL-2.0 */
/*
 * K3 Ring Accelerator (RA) subsystem interface
 *
 * Copyright (C) 2019 Texas Instruments Incorporated - https://www.ti.com
 */

// Translation of the C header. Linux-provided types and declarations are
// expected to be supplied by the surrounding translation environment.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ti_sci_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct k3_ringacc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct k3_ring {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum k3_ring_mode {
    K3_RINGACC_RING_MODE_RING = 0,
    K3_RINGACC_RING_MODE_MESSAGE,
    K3_RINGACC_RING_MODE_CREDENTIALS,
    K3_RINGACC_RING_MODE_INVALID,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum k3_ring_size {
    K3_RINGACC_RING_ELSIZE_4 = 0,
    K3_RINGACC_RING_ELSIZE_8,
    K3_RINGACC_RING_ELSIZE_16,
    K3_RINGACC_RING_ELSIZE_32,
    K3_RINGACC_RING_ELSIZE_64,
    K3_RINGACC_RING_ELSIZE_128,
    K3_RINGACC_RING_ELSIZE_256,
    K3_RINGACC_RING_ELSIZE_INVALID,
}

pub const K3_RINGACC_RING_SHARED: u32 = 1u32 << 1;

#[repr(C)]
pub struct k3_ring_cfg {
    pub size: u32,
    pub elm_size: k3_ring_size,
    pub mode: k3_ring_mode,
    pub flags: u32,
    pub dma_dev: *mut device,
    pub asel: u32,
}

pub const K3_RINGACC_RING_ID_ANY: c_int = -1;
pub const K3_RINGACC_RING_USE_PROXY: u32 = 1u32 << 1;

extern "C" {
    pub fn of_k3_ringacc_get_by_phandle(
        np: *mut device_node,
        property: *const c_char,
    ) -> *mut k3_ringacc;

    pub fn k3_ringacc_request_ring(
        ringacc: *mut k3_ringacc,
        id: c_int,
        flags: u32,
    ) -> *mut k3_ring;

    pub fn k3_ringacc_request_rings_pair(
        ringacc: *mut k3_ringacc,
        fwd_id: c_int,
        compl_id: c_int,
        fwd_ring: *mut *mut k3_ring,
        compl_ring: *mut *mut k3_ring,
    ) -> c_int;

    pub fn k3_ringacc_ring_reset(ring: *mut k3_ring);
    pub fn k3_ringacc_ring_reset_dma(ring: *mut k3_ring, occ: u32);
    pub fn k3_ringacc_ring_free(ring: *mut k3_ring) -> c_int;
    pub fn k3_ringacc_get_ring_id(ring: *mut k3_ring) -> u32;
    pub fn k3_ringacc_get_ring_irq_num(ring: *mut k3_ring) -> c_int;
    pub fn k3_ringacc_ring_cfg(ring: *mut k3_ring, cfg: *mut k3_ring_cfg) -> c_int;
    pub fn k3_ringacc_ring_get_size(ring: *mut k3_ring) -> u32;
    pub fn k3_ringacc_ring_get_free(ring: *mut k3_ring) -> u32;
    pub fn k3_ringacc_ring_get_occ(ring: *mut k3_ring) -> u32;
    pub fn k3_ringacc_ring_is_full(ring: *mut k3_ring) -> u32;
    pub fn k3_ringacc_ring_push(ring: *mut k3_ring, elem: *mut c_void) -> c_int;
    pub fn k3_ringacc_ring_pop(ring: *mut k3_ring, elem: *mut c_void) -> c_int;
    pub fn k3_ringacc_ring_push_head(ring: *mut k3_ring, elem: *mut c_void) -> c_int;
    pub fn k3_ringacc_ring_pop_tail(ring: *mut k3_ring, elem: *mut c_void) -> c_int;
    pub fn k3_ringacc_get_tisci_dev_id(ring: *mut k3_ring) -> u32;

    pub fn k3_ringacc_dmarings_init(
        pdev: *mut platform_device,
        data: *mut k3_ringacc_init_data,
    ) -> *mut k3_ringacc;
}

#[repr(C)]
pub struct k3_ringacc_init_data {
    pub tisci: *const ti_sci_handle,
    pub tisci_dev_id: u32,
    pub num_rings: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
