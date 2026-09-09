/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  linux/include/linux/mtd/onenand.h
 *
 *  Copyright © 2005-2009 Samsung Electronics
 *  Kyungmin Park <kyungmin.park@samsung.com>
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.
use core::ffi::{c_char, c_int, c_uint};

pub struct mtd_info;
pub struct mtd_oob_ops;
pub struct mtd_partition;
pub struct completion;
pub struct spinlock_t;
pub struct wait_queue_head_t;
pub type flstate_t = c_int;
pub type loff_t = i64;
pub type size_t = usize;

pub const MAX_DIES: usize = 2;
pub const MAX_BUFFERRAM: usize = 2;

extern "C" {
    pub fn onenand_scan(mtd: *mut mtd_info, max_chips: c_int) -> c_int;
    pub fn onenand_release(mtd: *mut mtd_info);
}

#[repr(C)]
pub struct onenand_bufferram {
    pub blockpage: c_int,
}

#[repr(C)]
pub struct onenand_chip {
    pub base: *mut core::ffi::c_void,
    pub dies: c_uint,
    pub boundary: [c_uint; MAX_DIES],
    pub diesize: [loff_t; MAX_DIES],
    pub chipsize: c_uint,
    pub device_id: c_uint,
    pub version_id: c_uint,
    pub technology: c_uint,
    pub density_mask: c_uint,
    pub options: c_uint,
    pub badblockpos: c_uint,

    pub erase_shift: c_uint,
    pub page_shift: c_uint,
    pub page_mask: c_uint,
    pub writesize: c_uint,

    pub bufferram_index: c_uint,
    pub bufferram: [onenand_bufferram; MAX_BUFFERRAM],

    pub command: Option<unsafe extern "C" fn(*mut mtd_info, c_int, loff_t, size_t) -> c_int>,
    pub wait: Option<unsafe extern "C" fn(*mut mtd_info, c_int) -> c_int>,
    pub bbt_wait: Option<unsafe extern "C" fn(*mut mtd_info, c_int) -> c_int>,
    pub unlock_all: Option<unsafe extern "C" fn(*mut mtd_info)>,
    pub read_bufferram: Option<unsafe extern "C" fn(*mut mtd_info, c_int, *mut u8, c_int, size_t) -> c_int>,
    pub write_bufferram: Option<unsafe extern "C" fn(*mut mtd_info, c_int, *const u8, c_int, size_t) -> c_int>,
    pub read_word: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> u16>,
    pub write_word: Option<unsafe extern "C" fn(u16, *mut core::ffi::c_void)>,
    pub mmcontrol: Option<unsafe extern "C" fn(*mut mtd_info, c_int)>,
    pub chip_probe: Option<unsafe extern "C" fn(*mut mtd_info) -> c_int>,
    pub block_markbad: Option<unsafe extern "C" fn(*mut mtd_info, loff_t) -> c_int>,
    pub scan_bbt: Option<unsafe extern "C" fn(*mut mtd_info) -> c_int>,
    pub enable: Option<unsafe extern "C" fn(*mut mtd_info) -> c_int>,
    pub disable: Option<unsafe extern "C" fn(*mut mtd_info) -> c_int>,

    pub complete: completion,
    pub irq: c_int,
    pub chip_lock: spinlock_t,
    pub wq: wait_queue_head_t,
    pub state: flstate_t,
    pub page_buf: *mut u8,
    pub oob_buf: *mut u8,
    // CONFIG_MTD_ONENAND_VERIFY_WRITE conditionally supplies this field.
    #[cfg(CONFIG_MTD_ONENAND_VERIFY_WRITE)]
    pub verify_buf: *mut u8,
    pub subpagesize: c_int,
    pub bbm: *mut core::ffi::c_void,
    pub priv_: *mut core::ffi::c_void,
    pub ongoing: c_uint,
}

pub const ONENAND_PAGES_PER_BLOCK: c_uint = 1 << 6;
pub const ONENAND_BADBLOCK_POS: c_uint = 0;

pub const ONENAND_HAS_CONT_LOCK: c_uint = 0x0001;
pub const ONENAND_HAS_UNLOCK_ALL: c_uint = 0x0002;
pub const ONENAND_HAS_2PLANE: c_uint = 0x0004;
pub const ONENAND_HAS_4KB_PAGE: c_uint = 0x0008;
pub const ONENAND_HAS_CACHE_PROGRAM: c_uint = 0x0010;
pub const ONENAND_HAS_NOP_1: c_uint = 0x0020;
pub const ONENAND_SKIP_UNLOCK_CHECK: c_uint = 0x0100;
pub const ONENAND_PAGEBUF_ALLOC: c_uint = 0x1000;
pub const ONENAND_OOBBUF_ALLOC: c_uint = 0x2000;
pub const ONENAND_SKIP_INITIAL_UNLOCKING: c_uint = 0x4000;

pub const ONENAND_MFR_SAMSUNG: c_uint = 0xec;
pub const ONENAND_MFR_NUMONYX: c_uint = 0x20;

#[repr(C)]
pub struct onenand_manufacturers {
    pub id: c_int,
    pub name: *mut c_char,
}

extern "C" {
    pub fn onenand_bbt_read_oob(mtd: *mut mtd_info, from: loff_t, ops: *mut mtd_oob_ops) -> c_int;
    pub fn onenand_block(this: *mut onenand_chip, addr: loff_t) -> c_uint;
    pub fn onenand_addr(this: *mut onenand_chip, block: c_int) -> loff_t;
    pub fn flexonenand_region(mtd: *mut mtd_info, addr: loff_t) -> c_int;
}

#[repr(C)]
pub struct onenand_platform_data {
    pub mmcontrol: Option<unsafe extern "C" fn(*mut mtd_info, c_int)>,
    pub read_bufferram: Option<unsafe extern "C" fn(*mut mtd_info, c_int, *mut u8, c_int, size_t) -> c_int>,
    pub parts: *mut mtd_partition,
    pub nr_parts: c_uint,
}

// Helper macros retained as direct Rust operations.
#[inline]
pub unsafe fn ONENAND_CURRENT_BUFFERRAM(this: *const onenand_chip) -> c_uint { (*this).bufferram_index }
#[inline]
pub unsafe fn ONENAND_NEXT_BUFFERRAM(this: *const onenand_chip) -> c_uint { (*this).bufferram_index ^ 1 }
#[inline]
pub unsafe fn ONENAND_SET_NEXT_BUFFERRAM(this: *mut onenand_chip) { (*this).bufferram_index ^= 1; }
#[inline]
pub unsafe fn ONENAND_SET_PREV_BUFFERRAM(this: *mut onenand_chip) { (*this).bufferram_index ^= 1; }
#[inline]
pub unsafe fn ONENAND_SET_BUFFERRAM0(this: *mut onenand_chip) { (*this).bufferram_index = 0; }
#[inline]
pub unsafe fn ONENAND_SET_BUFFERRAM1(this: *mut onenand_chip) { (*this).bufferram_index = 1; }
#[inline]
pub unsafe fn FLEXONENAND(this: *const onenand_chip) -> c_uint { (*this).device_id & DEVICE_IS_FLEXONENAND }
#[inline]
pub unsafe fn ONENAND_GET_SYS_CFG1(this: *mut onenand_chip) -> u16 {
    ((*this).read_word.unwrap())((*this).base.add(ONENAND_REG_SYS_CFG1 as usize))
}
#[inline]
pub unsafe fn ONENAND_SET_SYS_CFG1(v: u16, this: *mut onenand_chip) {
    ((*this).write_word.unwrap())(v, (*this).base.add(ONENAND_REG_SYS_CFG1 as usize));
}
#[inline]
pub unsafe fn ONENAND_CHECK_BYTE_ACCESS(addr: usize) -> usize { addr & 0x1 }
#[inline]
pub unsafe fn ONENAND_IS_DDP(this: *const onenand_chip) -> c_uint { (*this).device_id & ONENAND_DEVICE_IS_DDP }
#[inline]
pub unsafe fn ONENAND_IS_MLC(this: *const onenand_chip) -> c_uint { (*this).technology & ONENAND_TECHNOLOGY_IS_MLC }
#[inline]
pub unsafe fn ONENAND_IS_2PLANE(this: *const onenand_chip) -> c_uint {
    // CONFIG_MTD_ONENAND_2X_PROGRAM selects the feature-bearing macro.
    (*this).options & ONENAND_HAS_2PLANE
}
#[inline]
pub unsafe fn ONENAND_IS_CACHE_PROGRAM(this: *const onenand_chip) -> c_uint { (*this).options & ONENAND_HAS_CACHE_PROGRAM }
#[inline]
pub unsafe fn ONENAND_IS_NOP_1(this: *const onenand_chip) -> c_uint { (*this).options & ONENAND_HAS_NOP_1 }
#[inline]
pub unsafe fn ONENAND_IS_4KB_PAGE(this: *const onenand_chip) -> c_uint { (*this).options & ONENAND_HAS_4KB_PAGE }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
