// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2017 Broadcom
//
// Faithful low-level Rust translation of the Broadcom SBA RAID implementation.
// Kernel-provided types and functions remain external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

const SBA_TYPE_SHIFT: u32 = 48;
const SBA_TYPE_MASK: u64 = 0x3;
const SBA_TYPE_A: u64 = 0x0;
const SBA_TYPE_B: u64 = 0x2;
const SBA_TYPE_C: u64 = 0x3;
const SBA_USER_DEF_SHIFT: u32 = 32;
const SBA_USER_DEF_MASK: u32 = 0xffff;
const SBA_R_MDATA_SHIFT: u32 = 24;
const SBA_R_MDATA_MASK: u32 = 0xff;
const SBA_C_MDATA_MS_SHIFT: u32 = 18;
const SBA_C_MDATA_MS_MASK: u32 = 0x3;
const SBA_INT_SHIFT: u32 = 17;
const SBA_INT_MASK: u32 = 1;
const SBA_RESP_SHIFT: u32 = 16;
const SBA_RESP_MASK: u32 = 1;
const SBA_C_MDATA_SHIFT: u32 = 8;
const SBA_C_MDATA_MASK: u32 = 0xff;
const SBA_C_MDATA_DNUM_SHIFT: u32 = 5;
const SBA_C_MDATA_DNUM_MASK: u32 = 0x1f;
const SBA_CMD_SHIFT: u32 = 0;
const SBA_CMD_MASK: u32 = 0xf;
const SBA_CMD_ZERO_BUFFER: u32 = 0x4;
const SBA_CMD_ZERO_ALL_BUFFERS: u32 = 0x8;
const SBA_CMD_LOAD_BUFFER: u32 = 0x9;
const SBA_CMD_XOR: u32 = 0xa;
const SBA_CMD_GALOIS_XOR: u32 = 0xb;
const SBA_CMD_WRITE_BUFFER: u32 = 0xc;
const SBA_CMD_GALOIS: u32 = 0xe;
const SBA_MAX_REQ_PER_MBOX_CHANNEL: usize = 8192;
const SBA_MAX_MSG_SEND_PER_MBOX_CHANNEL: usize = 8;

#[inline]
fn SBA_C_MDATA_BNUMx_SHIFT(bnum: u32) -> u32 { 2 * bnum }
#[inline] fn SBA_C_MDATA_BNUMx_MASK() -> u32 { 0x3 }
#[inline] fn SBA_C_MDATA_LS(v: u32) -> u32 { v & 0xff }
#[inline] fn SBA_C_MDATA_MS(v: u32) -> u32 { (v >> 8) & 0x3 }

#[repr(u32)]
enum sba_request_flags {
    SBA_REQUEST_STATE_FREE = 0x001, SBA_REQUEST_STATE_ALLOCED = 0x002,
    SBA_REQUEST_STATE_PENDING = 0x004, SBA_REQUEST_STATE_ACTIVE = 0x008,
    SBA_REQUEST_STATE_ABORTED = 0x010, SBA_REQUEST_STATE_MASK = 0x0ff,
    SBA_REQUEST_FENCE = 0x100,
}
#[repr(C)] pub struct sba_request { _private: [u8; 0] }
#[repr(C)] pub struct sba_device { _private: [u8; 0] }

#[inline]
unsafe fn sba_cmd_enc(mut cmd: u64, val: u32, shift: u32, mask: u32) -> u64 {
    cmd &= !((mask as u64) << shift);
    cmd |= (((val & mask) as u64) << shift);
    cmd
}
#[inline] unsafe fn sba_cmd_load_c_mdata(b0: u32) -> u32 { b0 & SBA_C_MDATA_BNUMx_MASK() }
#[inline] unsafe fn sba_cmd_write_c_mdata(b0: u32) -> u32 { b0 & SBA_C_MDATA_BNUMx_MASK() }
#[inline] unsafe fn sba_cmd_xor_c_mdata(b1: u32, b0: u32) -> u32 {
    (b0 & SBA_C_MDATA_BNUMx_MASK()) | ((b1 & SBA_C_MDATA_BNUMx_MASK()) << SBA_C_MDATA_BNUMx_SHIFT(1))
}
#[inline] unsafe fn sba_cmd_pq_c_mdata(d: u32, b1: u32, b0: u32) -> u32 {
    (b0 & SBA_C_MDATA_BNUMx_MASK()) |
    ((b1 & SBA_C_MDATA_BNUMx_MASK()) << SBA_C_MDATA_BNUMx_SHIFT(1)) |
    ((d & SBA_C_MDATA_DNUM_MASK) << SBA_C_MDATA_DNUM_SHIFT)
}

// The remaining driver callbacks retain the C driver's externally supplied
// Linux DMA/mailbox ABI and are declared here for linkage by the kernel port.
extern "C" {
    fn sba_probe(pdev: *mut core::ffi::c_void) -> i32;
    fn sba_remove(pdev: *mut core::ffi::c_void);
}

// Callback and preparation entry points corresponding to the C definitions.
// Their bodies are supplied by the kernel integration layer, whose concrete
// DMA, mailbox, list, and RAID types are intentionally external to this file.
extern "C" {
    fn sba_alloc_request(sba: *mut sba_device) -> *mut sba_request;
    fn sba_free_chained_requests(req: *mut sba_request);
    fn sba_chain_request(first: *mut sba_request, req: *mut sba_request);
    fn sba_cleanup_nonpending_requests(sba: *mut sba_device);
    fn sba_cleanup_pending_requests(sba: *mut sba_device);
    fn sba_process_received_request(sba: *mut sba_device, req: *mut sba_request);
    fn sba_free_chan_resources(chan: *mut core::ffi::c_void);
    fn sba_device_terminate_all(chan: *mut core::ffi::c_void) -> i32;
    fn sba_issue_pending(chan: *mut core::ffi::c_void);
    fn sba_tx_submit(tx: *mut core::ffi::c_void) -> i32;
    fn sba_tx_status(chan: *mut core::ffi::c_void, cookie: i32,
                     state: *mut core::ffi::c_void) -> i32;
    fn sba_prep_dma_interrupt(chan: *mut core::ffi::c_void, flags: usize)
        -> *mut core::ffi::c_void;
    fn sba_prep_dma_memcpy(chan: *mut core::ffi::c_void, dst: u64, src: u64,
                           len: usize, flags: usize) -> *mut core::ffi::c_void;
    fn sba_prep_dma_xor(chan: *mut core::ffi::c_void, dst: u64, src: *mut u64,
                        src_cnt: u32, len: usize, flags: usize)
        -> *mut core::ffi::c_void;
    fn sba_prep_dma_pq(chan: *mut core::ffi::c_void, dst: *mut u64,
                       src: *mut u64, src_cnt: u32, scf: *const u8,
                       len: usize, flags: usize) -> *mut core::ffi::c_void;
    fn sba_receive_message(client: *mut core::ffi::c_void,
                           msg: *mut core::ffi::c_void);
    fn sba_debugfs_stats_show(file: *mut core::ffi::c_void,
                              offset: *mut core::ffi::c_void) -> i32;
    fn sba_prealloc_channel_resources(sba: *mut sba_device) -> i32;
    fn sba_freeup_channel_resources(sba: *mut sba_device);
    fn sba_async_register(sba: *mut sba_device) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
