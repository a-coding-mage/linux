/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2007 Krzysztof Halasa <khc@pm.waw.pl>
 */

//! Rust translation of `ixp4xx/qmgr.h`.
//! The Linux I/O and kernel includes are supplied by external dependencies.

use core::ffi::{c_char, c_int, c_void};

pub const DEBUG_QMGR: c_int = 0;

pub const HALF_QUEUES: usize = 32;
pub const QUEUES: usize = 64;
pub const MAX_QUEUE_LENGTH: usize = 4; // in dwords

pub const QUEUE_STAT1_EMPTY: u32 = 1; // queue status bits
pub const QUEUE_STAT1_NEARLY_EMPTY: u32 = 2;
pub const QUEUE_STAT1_NEARLY_FULL: u32 = 4;
pub const QUEUE_STAT1_FULL: u32 = 8;
pub const QUEUE_STAT2_UNDERFLOW: u32 = 1;
pub const QUEUE_STAT2_OVERFLOW: u32 = 2;

pub const QUEUE_WATERMARK_0_ENTRIES: u32 = 0;
pub const QUEUE_WATERMARK_1_ENTRY: u32 = 1;
pub const QUEUE_WATERMARK_2_ENTRIES: u32 = 2;
pub const QUEUE_WATERMARK_4_ENTRIES: u32 = 3;
pub const QUEUE_WATERMARK_8_ENTRIES: u32 = 4;
pub const QUEUE_WATERMARK_16_ENTRIES: u32 = 5;
pub const QUEUE_WATERMARK_32_ENTRIES: u32 = 6;
pub const QUEUE_WATERMARK_64_ENTRIES: u32 = 7;

// queue interrupt request conditions
pub const QUEUE_IRQ_SRC_EMPTY: c_int = 0;
pub const QUEUE_IRQ_SRC_NEARLY_EMPTY: c_int = 1;
pub const QUEUE_IRQ_SRC_NEARLY_FULL: c_int = 2;
pub const QUEUE_IRQ_SRC_FULL: c_int = 3;
pub const QUEUE_IRQ_SRC_NOT_EMPTY: c_int = 4;
pub const QUEUE_IRQ_SRC_NOT_NEARLY_EMPTY: c_int = 5;
pub const QUEUE_IRQ_SRC_NOT_NEARLY_FULL: c_int = 6;
pub const QUEUE_IRQ_SRC_NOT_FULL: c_int = 7;

#[repr(C)]
pub struct qmgr_regs {
    pub acc: [[u32; MAX_QUEUE_LENGTH]; QUEUES], // 0x000 - 0x3FF
    pub stat1: [u32; 4], // 0x400 - 0x40F
    pub stat2: [u32; 2], // 0x410 - 0x417
    pub statne_h: u32, // 0x418 - queue nearly empty
    pub statf_h: u32, // 0x41C - queue full
    pub irqsrc: [u32; 4], // 0x420 - IRC source
    pub irqen: [u32; 2], // 0x430 - IRQ enabled
    pub irqstat: [u32; 2], // 0x438 - IRQ access only
    pub reserved: [u32; 1776],
    pub sram: [u32; 2048], // 0x2000 - 0x3FFF - config and buffer
}

unsafe extern "C" {
    pub fn qmgr_put_entry(queue: u32, val: u32);
    pub fn qmgr_get_entry(queue: u32) -> u32;
    pub fn qmgr_stat_empty(queue: u32) -> c_int;
    pub fn qmgr_stat_below_low_watermark(queue: u32) -> c_int;
    pub fn qmgr_stat_full(queue: u32) -> c_int;
    pub fn qmgr_stat_overflow(queue: u32) -> c_int;
    pub fn qmgr_release_queue(queue: u32);
    pub fn qmgr_set_irq(queue: u32, src: c_int, handler: Option<unsafe extern "C" fn(*mut c_void)>, pdev: *mut c_void);
    pub fn qmgr_enable_irq(queue: u32);
    pub fn qmgr_disable_irq(queue: u32);

    // request_ and release_queue() must be called from non-IRQ context
    pub fn __qmgr_request_queue(queue: u32, len: u32, nearly_empty_watermark: u32, nearly_full_watermark: u32) -> c_int;
}

// DEBUG_QMGR is zero in this header; the debug-only declarations are retained
// here as conditional Rust declarations for builds that enable that condition.
#[cfg(feature = "debug_qmgr")]
extern "C" {
    pub static mut qmgr_queue_descs: [[c_char; 32]; QUEUES];
    pub fn qmgr_request_queue_debug(
        queue: u32,
        len: u32,
        nearly_empty_watermark: u32,
        nearly_full_watermark: u32,
        desc_format: *const c_char,
        name: *const c_char,
    ) -> c_int;
}

#[inline]
pub unsafe fn qmgr_request_queue(
    queue: u32,
    len: u32,
    nearly_empty_watermark: u32,
    nearly_full_watermark: u32,
    _desc_format: *const c_char,
    _name: *const c_char,
) -> c_int {
    __qmgr_request_queue(queue, len, nearly_empty_watermark, nearly_full_watermark)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
