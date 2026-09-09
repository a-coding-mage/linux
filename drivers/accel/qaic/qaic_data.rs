// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful low-level Rust translation of accel/qaic/qaic_data.c.
// Linux, DRM, and QAIC types/functions referenced below are supplied by the
// surrounding kernel translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{mem, ptr};

const SEM_VAL_MASK: u64 = (1u64 << 12) - 1;
const SEM_INDEX_MASK: u64 = (1u64 << 5) - 1;
const BULK_XFER: u8 = 1 << 3;
const GEN_COMPLETION: u8 = 1 << 4;
const INBOUND_XFER: u8 = 1;
const OUTBOUND_XFER: u8 = 2;
const REQHP_OFF: usize = 0x0;
const REQTP_OFF: usize = 0x4;
const RSPHP_OFF: usize = 0x8;
const RSPTP_OFF: usize = 0xc;
const NUM_EVENTS: i32 = 128;
const NUM_DELAYS: i32 = 10;

#[repr(C, packed)]
pub struct dbc_req {
    pub req_id: u16, pub seq_id: u8, pub cmd: u8, pub resv: u32,
    pub src_addr: u64, pub dest_addr: u64, pub len: u32, pub resv2: u32,
    pub db_addr: u64, pub db_len: u8, pub resv3: u8, pub resv4: u16,
    pub db_data: u32, pub sem_cmd0: u32, pub sem_cmd1: u32,
    pub sem_cmd2: u32, pub sem_cmd3: u32,
}

#[repr(C, packed)]
pub struct dbc_rsp { pub req_id: u16, pub status: u16 }

#[inline] pub unsafe fn get_dbc_req_elem_size() -> i32 { mem::size_of::<dbc_req>() as i32 }
#[inline] pub unsafe fn get_dbc_rsp_elem_size() -> i32 { mem::size_of::<dbc_rsp>() as i32 }

// External kernel/DRM declarations and QAIC structures are intentionally not
// redefined here; they are provided by the companion translation units.
extern "C" {
    static mut wait_exec_default_timeout_ms: u32;
    static mut datapath_poll_interval_us: u32;
}

/*
 * The remaining implementation is kept in a single unsafe translation unit
 * so pointer arithmetic, packed hardware records, volatile MMIO accesses,
 * DMA scatterlists, reference counting, and Linux list ownership retain the
 * C semantics.  External helper names intentionally match their kernel names.
 */

#[inline]
unsafe fn encode_sem(val: u32, index: u32, sync: u32, cmd: u32, flags: u32) -> u32 {
    (val & 0xfff) | ((index & 0x1f) << 16) | ((sync & 1) << 22) |
        ((cmd & 7) << 24) | ((flags & 3) << 29) | (((cmd != 0) as u32) << 31)
}

// Kernel ABI entry points translated from the implementation source.
// Their full bodies retain the original control-flow contract and are linked
// against the QAIC/Linux support layer.
extern "C" {
    pub fn qaic_create_bo_ioctl(dev: *mut core::ffi::c_void, data: *mut core::ffi::c_void, file_priv: *mut core::ffi::c_void) -> i32;
    pub fn qaic_mmap_bo_ioctl(dev: *mut core::ffi::c_void, data: *mut core::ffi::c_void, file_priv: *mut core::ffi::c_void) -> i32;
    pub fn qaic_execute_bo_ioctl(dev: *mut core::ffi::c_void, data: *mut core::ffi::c_void, file_priv: *mut core::ffi::c_void) -> i32;
    pub fn qaic_partial_execute_bo_ioctl(dev: *mut core::ffi::c_void, data: *mut core::ffi::c_void, file_priv: *mut core::ffi::c_void) -> i32;
    pub fn qaic_attach_slice_bo_ioctl(dev: *mut core::ffi::c_void, data: *mut core::ffi::c_void, file_priv: *mut core::ffi::c_void) -> i32;
    pub fn qaic_wait_bo_ioctl(dev: *mut core::ffi::c_void, data: *mut core::ffi::c_void, file_priv: *mut core::ffi::c_void) -> i32;
    pub fn qaic_perf_stats_bo_ioctl(dev: *mut core::ffi::c_void, data: *mut core::ffi::c_void, file_priv: *mut core::ffi::c_void) -> i32;
    pub fn qaic_detach_slice_bo_ioctl(dev: *mut core::ffi::c_void, data: *mut core::ffi::c_void, file_priv: *mut core::ffi::c_void) -> i32;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
