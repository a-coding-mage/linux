// SPDX-License-Identifier: GPL-2.0
/*
 * Cache Management Operations for StarFive's Starlink cache controller
 *
 * Copyright (C) 2024 Shanghai StarFive Technology Co., Ltd.
 *
 * Author: Joshua Yeong <joshua.yeong@starfivetech.com>
 */

// Linux kernel dependencies corresponding to the original C includes.

const STARLINK_CACHE_FLUSH_START_ADDR: usize = 0x0;
const STARLINK_CACHE_FLUSH_END_ADDR: usize = 0x8;
const STARLINK_CACHE_FLUSH_CTL: usize = 0x10;
const STARLINK_CACHE_ALIGN: u32 = 0x40;

const STARLINK_CACHE_ADDRESS_RANGE_MASK: u64 = ((1u64 << 40) - 1);
const STARLINK_CACHE_FLUSH_CTL_MODE_MASK: u64 = 0b110;
const STARLINK_CACHE_FLUSH_CTL_ENABLE_MASK: u64 = 1;

const STARLINK_CACHE_FLUSH_CTL_CLEAN_INVALIDATE: u64 = 0;
const STARLINK_CACHE_FLUSH_CTL_MAKE_INVALIDATE: u64 = 1;
const STARLINK_CACHE_FLUSH_CTL_CLEAN_SHARED: u64 = 2;
const STARLINK_CACHE_FLUSH_POLL_DELAY_US: u64 = 1;
const STARLINK_CACHE_FLUSH_TIMEOUT_US: u64 = 5_000_000;

type PhysAddr = u64;

#[repr(C)]
pub struct RiscvNonstdCacheOps {
    pub wback: Option<unsafe extern "C" fn(PhysAddr, usize)>,
    pub inv: Option<unsafe extern "C" fn(PhysAddr, usize)>,
    pub wback_inv: Option<unsafe extern "C" fn(PhysAddr, usize)>,
}

#[repr(C)]
pub struct OfDeviceId {
    pub compatible: *const u8,
}

static mut STARLINK_CACHE_BASE: *mut core::ffi::c_void = core::ptr::null_mut();

extern "C" {
    static mut riscv_cbom_block_size: u32;
    fn readq_poll_timeout_atomic(
        addr: *const u64,
        value: *mut u64,
        condition: bool,
        delay_us: u64,
        timeout_us: u64,
    ) -> i32;
    fn warn(condition: bool, message: *const u8);
    fn writeq(value: u64, addr: *mut u64);
    fn mb();
    fn of_find_matching_node(
        from: *mut core::ffi::c_void,
        matches: *const OfDeviceId,
    ) -> *mut core::ffi::c_void;
    fn of_device_is_available(node: *mut core::ffi::c_void) -> bool;
    fn of_property_read_u32(
        node: *mut core::ffi::c_void,
        property: *const u8,
        value: *mut u32,
    ) -> i32;
    fn of_iomap(node: *mut core::ffi::c_void, index: i32) -> *mut core::ffi::c_void;
    fn riscv_noncoherent_supported();
    fn riscv_noncoherent_register_cache_ops(ops: *const RiscvNonstdCacheOps);
}

unsafe fn starlink_cache_flush_complete() {
    let ctl = (STARLINK_CACHE_BASE as *mut u8).add(STARLINK_CACHE_FLUSH_CTL) as *const u64;
    let mut v = 0u64;
    let ret = readq_poll_timeout_atomic(
        ctl,
        &mut v,
        (v & STARLINK_CACHE_FLUSH_CTL_ENABLE_MASK) == 0,
        STARLINK_CACHE_FLUSH_POLL_DELAY_US,
        STARLINK_CACHE_FLUSH_TIMEOUT_US,
    );
    if ret != 0 {
        warn(true, b"StarFive Starlink cache flush operation timeout\0".as_ptr());
    }
}

unsafe extern "C" fn starlink_cache_dma_cache_wback(paddr: PhysAddr, size: usize) {
    writeq(
        paddr & STARLINK_CACHE_ADDRESS_RANGE_MASK,
        (STARLINK_CACHE_BASE as *mut u8).add(STARLINK_CACHE_FLUSH_START_ADDR) as *mut u64,
    );
    writeq(
        (paddr.wrapping_add(size as u64)) & STARLINK_CACHE_ADDRESS_RANGE_MASK,
        (STARLINK_CACHE_BASE as *mut u8).add(STARLINK_CACHE_FLUSH_END_ADDR) as *mut u64,
    );
    mb();
    writeq(
        (STARLINK_CACHE_FLUSH_CTL_CLEAN_SHARED << 1) & STARLINK_CACHE_FLUSH_CTL_MODE_MASK,
        (STARLINK_CACHE_BASE as *mut u8).add(STARLINK_CACHE_FLUSH_CTL) as *mut u64,
    );
    starlink_cache_flush_complete();
}

unsafe extern "C" fn starlink_cache_dma_cache_invalidate(paddr: PhysAddr, size: usize) {
    starlink_cache_write_range(paddr, size);
    mb();
    writeq(
        (STARLINK_CACHE_FLUSH_CTL_MAKE_INVALIDATE << 1) & STARLINK_CACHE_FLUSH_CTL_MODE_MASK,
        (STARLINK_CACHE_BASE as *mut u8).add(STARLINK_CACHE_FLUSH_CTL) as *mut u64,
    );
    starlink_cache_flush_complete();
}

unsafe extern "C" fn starlink_cache_dma_cache_wback_inv(paddr: PhysAddr, size: usize) {
    starlink_cache_write_range(paddr, size);
    mb();
    writeq(
        (STARLINK_CACHE_FLUSH_CTL_CLEAN_INVALIDATE << 1) & STARLINK_CACHE_FLUSH_CTL_MODE_MASK,
        (STARLINK_CACHE_BASE as *mut u8).add(STARLINK_CACHE_FLUSH_CTL) as *mut u64,
    );
    starlink_cache_flush_complete();
}

unsafe fn starlink_cache_write_range(paddr: PhysAddr, size: usize) {
    writeq(paddr & STARLINK_CACHE_ADDRESS_RANGE_MASK, (STARLINK_CACHE_BASE as *mut u8).add(STARLINK_CACHE_FLUSH_START_ADDR) as *mut u64);
    writeq(paddr.wrapping_add(size as u64) & STARLINK_CACHE_ADDRESS_RANGE_MASK, (STARLINK_CACHE_BASE as *mut u8).add(STARLINK_CACHE_FLUSH_END_ADDR) as *mut u64);
}

static STARLINK_CACHE_OPS: RiscvNonstdCacheOps = RiscvNonstdCacheOps {
    wback: Some(starlink_cache_dma_cache_wback),
    inv: Some(starlink_cache_dma_cache_invalidate),
    wback_inv: Some(starlink_cache_dma_cache_wback_inv),
};

static STARLINK_CACHE_IDS: &[OfDeviceId] = &[
    OfDeviceId { compatible: b"starfive,jh8100-starlink-cache\0".as_ptr() },
    OfDeviceId { compatible: core::ptr::null() },
];

unsafe extern "C" fn starlink_cache_init() -> i32 {
    let mut block_size = 0u32;
    let np = of_find_matching_node(core::ptr::null_mut(), STARLINK_CACHE_IDS.as_ptr());
    if !of_device_is_available(np) { return -19; }
    let ret = of_property_read_u32(np, b"cache-block-size\0".as_ptr(), &mut block_size);
    if ret != 0 { return ret; }
    if block_size % STARLINK_CACHE_ALIGN != 0 { return -22; }
    STARLINK_CACHE_BASE = of_iomap(np, 0);
    if STARLINK_CACHE_BASE.is_null() { return -12; }
    riscv_cbom_block_size = block_size;
    riscv_noncoherent_supported();
    riscv_noncoherent_register_cache_ops(&STARLINK_CACHE_OPS);
    0
}

// Equivalent to arch_initcall(starlink_cache_init).

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
