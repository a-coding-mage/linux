/***********************license start***************
 * Author: Cavium Networks
 *
 * Contact: support@caviumnetworks.com
 * This file is part of the OCTEON SDK
 *
 * Copyright (c) 2003-2008 Cavium Networks
 *
 * This file is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2, as published by
 * the Free Software Foundation.
 ***********************license end**************************************/

//! Interface to the hardware Free Pool Allocator.

// Includes from the C header are intentionally omitted; their symbols are
// supplied by the surrounding translation unit.

pub const CVMX_FPA_NUM_POOLS: usize = 8;
pub const CVMX_FPA_MIN_BLOCK_SIZE: u64 = 128;
pub const CVMX_FPA_ALIGNMENT: u64 = 128;

/// Structure describing the data format used for stores to the FPA.
#[repr(C)]
pub union cvmx_fpa_iobdma_data_t {
    pub u64: u64,
    pub s: cvmx_fpa_iobdma_data_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_fpa_iobdma_data_s {
    pub bits: u64,
}

impl cvmx_fpa_iobdma_data_s {
    pub fn scraddr(&self) -> u64 { self.bits & 0xff }
    pub fn set_scraddr(&mut self, v: u64) { self.bits = (self.bits & !0xff) | (v & 0xff); }
    pub fn len(&self) -> u64 { (self.bits >> 8) & 0xff }
    pub fn set_len(&mut self, v: u64) { self.bits = (self.bits & !(0xff << 8)) | ((v & 0xff) << 8); }
    pub fn did(&self) -> u64 { (self.bits >> 16) & 0xff }
    pub fn set_did(&mut self, v: u64) { self.bits = (self.bits & !(0xff << 16)) | ((v & 0xff) << 16); }
    pub fn addr(&self) -> u64 { (self.bits >> 24) & ((1u64 << 40) - 1) }
    pub fn set_addr(&mut self, v: u64) { self.bits = (self.bits & !(((1u64 << 40) - 1) << 24)) | ((v & ((1u64 << 40) - 1)) << 24); }
}

/// Structure describing the current state of a FPA pool.
#[repr(C)]
pub struct cvmx_fpa_pool_info_t {
    pub name: *const core::ffi::c_char,
    pub size: u64,
    pub base: *mut core::ffi::c_void,
    pub starting_element_count: u64,
}

extern "C" {
    pub static mut cvmx_fpa_pool_info: [cvmx_fpa_pool_info_t; CVMX_FPA_NUM_POOLS];
    pub fn cvmx_fpa_shutdown_pool(pool: u64) -> u64;
    pub fn cvmx_fpa_get_block_size(pool: u64) -> u64;
    pub fn cvmx_read_csr(address: u64) -> u64;
    pub fn cvmx_write_csr(address: u64, value: u64);
    pub fn cvmx_octeon_is_pass1() -> bool;
    pub fn cvmx_dprintf(fmt: *const core::ffi::c_char, ...);
    pub fn cvmx_phys_to_ptr(address: u64) -> *mut core::ffi::c_void;
    pub fn cvmx_ptr_to_phys(ptr: *mut core::ffi::c_void) -> u64;
    pub fn cvmx_send_single(value: u64);
    pub fn cvmx_write_io(address: u64, value: u64);
    pub fn __delay(cycles: u32);
}

pub const CVMX_FPA_CTL_STATUS: u64 = 0; // supplied by cvmx-fpa-defs.h
pub const CVMX_FPA_FPF1_MARKS: u64 = 0; // supplied by cvmx-fpa-defs.h

#[inline]
pub unsafe fn cvmx_fpa_get_name(pool: u64) -> *const core::ffi::c_char {
    cvmx_fpa_pool_info[pool as usize].name
}

#[inline]
pub unsafe fn cvmx_fpa_get_base(pool: u64) -> *mut core::ffi::c_void {
    cvmx_fpa_pool_info[pool as usize].base
}

#[inline]
pub unsafe fn cvmx_fpa_is_member(pool: u64, ptr: *mut core::ffi::c_void) -> i32 {
    let info = &cvmx_fpa_pool_info[pool as usize];
    let start = info.base as usize;
    let p = ptr as usize;
    (if p >= start && p < start.wrapping_add((info.size.wrapping_mul(info.starting_element_count)) as usize) { 1 } else { 0 })
}

#[inline]
pub unsafe fn cvmx_fpa_enable() {
    // CVMX_FPA_CTL_STATUS fields are defined by cvmx-fpa-defs.h.
    let mut status = cvmx_read_csr(CVMX_FPA_CTL_STATUS);
    if (status & 1) != 0 {
        cvmx_dprintf(b"Warning: Enabling FPA when FPA already enabled.\n\0".as_ptr() as *const core::ffi::c_char);
    }
    if cvmx_octeon_is_pass1() {
        for i in 1u64..8 {
            let address = CVMX_FPA_FPF1_MARKS.wrapping_add((i - 1).wrapping_mul(8));
            let mut marks = cvmx_read_csr(address);
            marks = (marks & !(0xff << 0)) | (0xe0 << 0);
            cvmx_write_csr(address, marks);
        }
        __delay(10);
    }
    status = 1;
    cvmx_write_csr(CVMX_FPA_CTL_STATUS, status);
}

#[inline]
pub unsafe fn cvmx_fpa_alloc(pool: u64) -> *mut core::ffi::c_void {
    let address = cvmx_read_csr(pool); // CVMX_ADDR_DID(CVMX_FULL_DID(CVMX_OCT_DID_FPA, pool))
    if address != 0 { cvmx_phys_to_ptr(address) } else { core::ptr::null_mut() }
}

#[inline]
pub unsafe fn cvmx_fpa_async_alloc(scr_addr: u64, pool: u64) {
    let mut data = cvmx_fpa_iobdma_data_s { bits: 0 };
    data.set_scraddr(scr_addr >> 3);
    data.set_len(1);
    data.set_did(pool); // CVMX_FULL_DID(CVMX_OCT_DID_FPA, pool)
    data.set_addr(0);
    cvmx_send_single(cvmx_fpa_iobdma_data_t { s: data }.u64);
}

#[inline]
pub unsafe fn cvmx_fpa_free_nosync(ptr: *mut core::ffi::c_void, pool: u64, num_cache_lines: u64) {
    let newptr = cvmx_ptr_to_phys(ptr); // cvmx_addr_t with sfilldidspace.didspace set below
    let _ = pool; // CVMX_ADDR_DIDSPACE(CVMX_FULL_DID(CVMX_OCT_DID_FPA, pool))
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    cvmx_write_io(newptr, num_cache_lines);
}

#[inline]
pub unsafe fn cvmx_fpa_free(ptr: *mut core::ffi::c_void, pool: u64, num_cache_lines: u64) {
    let newptr = cvmx_ptr_to_phys(ptr);
    let _ = pool; // CVMX_ADDR_DIDSPACE(CVMX_FULL_DID(CVMX_OCT_DID_FPA, pool))
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
    cvmx_write_io(newptr, num_cache_lines);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
