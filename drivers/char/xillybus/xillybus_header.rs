/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * linux/drivers/misc/xillybus.h
 *
 * Copyright 2011 Xillybus Ltd, http://xillybus.com
 *
 * Header file for the Xillybus FPGA/host framework.
 */

/* Linux headers supplied by the surrounding build environment. */

use core::ffi::c_void;

pub type DmaAddrT = u64;
pub type U32 = u32;
pub type IrqreturnT = i32;

/* Opaque types supplied by the surrounding Linux environment. */
pub enum Device {}
pub enum Module {}
pub enum SpinlockT {}
pub enum Mutex {}
pub enum WaitQueueHeadT {}
pub enum DelayedWork {}

pub struct XillyEndpointHardware;

#[repr(C)]
pub struct XillyBuffer {
    pub addr: *mut c_void,
    pub dma_addr: DmaAddrT,
    pub end_offset: i32, /* Counting elements, not bytes */
}

#[repr(C)]
pub struct XillyIdtHandle {
    pub chandesc: *mut u8,
    pub names: *mut u8,
    pub names_len: i32,
    pub entries: i32,
}

/*
 * Read-write confusion: wr_* and rd_* notation sticks to FPGA view, so
 * wr_* buffers are those consumed by read(), since the FPGA writes to them
 * and vice versa.
 */

#[repr(C)]
pub struct XillyChannel {
    pub endpoint: *mut XillyEndpoint,
    pub chan_num: i32,
    pub log2_element_size: i32,
    pub seekable: i32,

    pub wr_buffers: *mut *mut XillyBuffer, /* FPGA writes, driver reads! */
    pub num_wr_buffers: i32,
    pub wr_buf_size: u32, /* In bytes */
    pub wr_fpga_buf_idx: i32,
    pub wr_host_buf_idx: i32,
    pub wr_host_buf_pos: i32,
    pub wr_empty: i32,
    pub wr_ready: i32, /* Significant only when wr_empty == 1 */
    pub wr_sleepy: i32,
    pub wr_eof: i32,
    pub wr_hangup: i32,
    pub wr_spinlock: *mut SpinlockT,
    pub wr_mutex: *mut Mutex,
    pub wr_wait: *mut WaitQueueHeadT,
    pub wr_ready_wait: *mut WaitQueueHeadT,
    pub wr_ref_count: i32,
    pub wr_synchronous: i32,
    pub wr_allow_partial: i32,
    pub wr_exclusive_open: i32,
    pub wr_supports_nonempty: i32,

    pub rd_buffers: *mut *mut XillyBuffer, /* FPGA reads, driver writes! */
    pub num_rd_buffers: i32,
    pub rd_buf_size: u32, /* In bytes */
    pub rd_fpga_buf_idx: i32,
    pub rd_host_buf_pos: i32,
    pub rd_host_buf_idx: i32,
    pub rd_full: i32,
    pub rd_spinlock: *mut SpinlockT,
    pub rd_mutex: *mut Mutex,
    pub rd_wait: *mut WaitQueueHeadT,
    pub rd_ref_count: i32,
    pub rd_allow_partial: i32,
    pub rd_synchronous: i32,
    pub rd_exclusive_open: i32,
    pub rd_workitem: DelayedWork,
    pub rd_leftovers: [u8; 4],
}

#[repr(C)]
pub struct XillyEndpoint {
    pub dev: *mut Device,
    pub owner: *mut Module,

    pub dma_using_dac: i32, /* =1 if 64-bit DMA is used, =0 otherwise. */
    pub registers: *mut c_void,
    pub fatal_error: i32,

    pub register_mutex: Mutex,
    pub ep_wait: WaitQueueHeadT,

    pub num_channels: i32, /* EXCLUDING message buffer */
    pub channels: *mut *mut XillyChannel,
    pub msg_counter: i32,
    pub failed_messages: i32,
    pub idtlen: i32,

    pub msgbuf_addr: *mut U32,
    pub msgbuf_dma_addr: DmaAddrT,
    pub msg_buf_size: u32,
}

#[repr(C)]
pub struct XillyMapping {
    pub device: *mut Device,
    pub dma_addr: DmaAddrT,
    pub size: usize,
    pub direction: i32,
}

extern "C" {
    pub fn xillybus_isr(irq: i32, data: *mut c_void) -> IrqreturnT;

    pub fn xillybus_init_endpoint(dev: *mut Device) -> *mut XillyEndpoint;

    pub fn xillybus_endpoint_discovery(endpoint: *mut XillyEndpoint) -> i32;

    pub fn xillybus_endpoint_remove(endpoint: *mut XillyEndpoint);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
