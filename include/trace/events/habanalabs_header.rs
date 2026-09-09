/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright 2022-2023 HabanaLabs, Ltd.
 * All Rights Reserved.
 */

//! Rust translation of the Linux tracepoint header `habanalabs.h`.
//! The original declarations are tracepoint metadata; the external trace
//! registration/emission machinery is supplied by the surrounding kernel.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct Device {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HabanalabsMmuEntry {
    pub dname: *const c_char,
    pub virt_addr: u64,
    pub phys_addr: u64,
    pub page_size: u32,
    pub flush_pte: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HabanalabsDmaAllocEntry {
    pub dname: *const c_char,
    pub cpu_addr: u64,
    pub dma_addr: u64,
    pub size: u32,
    pub caller: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HabanalabsDmaMapEntry {
    pub dname: *const c_char,
    pub phys_addr: u64,
    pub dma_addr: u64,
    pub len: u32,
    pub dir: c_int,
    pub caller: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HabanalabsCommsEntry {
    pub dname: *const c_char,
    pub op_str: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HabanalabsRegAccessEntry {
    pub dname: *const c_char,
    pub addr: u32,
    pub val: u32,
}

// Original TP_PROTO declarations:
// struct device *dev, u64 virt_addr, u64 phys_addr, u32 page_size,
// bool flush_pte
// struct device *dev, u64 cpu_addr, u64 dma_addr, size_t size,
// const char *caller
// struct device *dev, u64 phys_addr, u64 dma_addr, size_t len,
// enum dma_data_direction dir, const char *caller
// struct device *dev, char *op_str
// struct device *dev, u32 addr, u32 val

extern "C" {
    // DEFINE_EVENT(habanalabs_mmu_template, habanalabs_mmu_map)
    pub fn habanalabs_mmu_map(
        dev: *mut Device, virt_addr: u64, phys_addr: u64, page_size: u32,
        flush_pte: bool,
    );
    // DEFINE_EVENT(habanalabs_mmu_template, habanalabs_mmu_unmap)
    pub fn habanalabs_mmu_unmap(
        dev: *mut Device, virt_addr: u64, phys_addr: u64, page_size: u32,
        flush_pte: bool,
    );

    pub fn habanalabs_dma_alloc(
        dev: *mut Device, cpu_addr: u64, dma_addr: u64, size: usize,
        caller: *const c_char,
    );
    pub fn habanalabs_dma_free(
        dev: *mut Device, cpu_addr: u64, dma_addr: u64, size: usize,
        caller: *const c_char,
    );

    pub fn habanalabs_dma_map_page(
        dev: *mut Device, phys_addr: u64, dma_addr: u64, len: usize,
        dir: c_int, caller: *const c_char,
    );
    pub fn habanalabs_dma_unmap_page(
        dev: *mut Device, phys_addr: u64, dma_addr: u64, len: usize,
        dir: c_int, caller: *const c_char,
    );

    pub fn habanalabs_comms_protocol_cmd(dev: *mut Device, op_str: *mut c_char);
    pub fn habanalabs_comms_send_cmd(dev: *mut Device, op_str: *mut c_char);
    pub fn habanalabs_comms_wait_status(dev: *mut Device, op_str: *mut c_char);
    pub fn habanalabs_comms_wait_status_done(dev: *mut Device, op_str: *mut c_char);

    pub fn habanalabs_rreg32(dev: *mut Device, addr: u32, val: u32);
    pub fn habanalabs_wreg32(dev: *mut Device, addr: u32, val: u32);
    pub fn habanalabs_elbi_read(dev: *mut Device, addr: u32, val: u32);
    pub fn habanalabs_elbi_write(dev: *mut Device, addr: u32, val: u32);
}

// `trace/define_trace.h` is intentionally omitted: it is a kernel build-time
// include that materializes the tracepoint definitions outside this header.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
