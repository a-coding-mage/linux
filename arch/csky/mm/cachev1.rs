// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.

/* Dependencies supplied by the surrounding kernel translation. */
extern "C" {
    fn mtcr(reg: *const core::ffi::c_char, value: u32);
    fn mfcr_ccr2() -> u32;
    fn mb();
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: usize);
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

/* for L1-cache */
const INS_CACHE: u32 = 1 << 0;
const DATA_CACHE: u32 = 1 << 1;
const CACHE_INV: u32 = 1 << 4;
const CACHE_CLR: u32 = 1 << 5;
const CACHE_OMS: u32 = 1 << 6;
const CACHE_ITS: u32 = 1 << 7;
const CACHE_LICF: u32 = 1 << 31;

/* for L2-cache */
const CR22_LEVEL_SHIFT: u32 = 1;
const CR22_SET_SHIFT: u32 = 7;
const CR22_WAY_SHIFT: u32 = 30;
const CR22_WAY_SHIFT_L2: u32 = 29;

static mut cache_lock: core::mem::MaybeUninit<spinlock_t> = core::mem::MaybeUninit::uninit();

unsafe fn cache_op_line(i: usize, val: u32) {
    mtcr(b"cr22\0".as_ptr() as *const core::ffi::c_char, i as u32);
    mtcr(b"cr17\0".as_ptr() as *const core::ffi::c_char, val);
}

const CCR2_L2E: u32 = 1 << 3;
unsafe fn cache_op_all(value: u32, l2: u32) {
    mtcr(b"cr17\0".as_ptr() as *const core::ffi::c_char, value | CACHE_CLR);
    mb();

    if l2 != 0 && (mfcr_ccr2() & CCR2_L2E) != 0 {
        mtcr(b"cr24\0".as_ptr() as *const core::ffi::c_char, value | CACHE_CLR);
        mb();
    }
}

unsafe fn cache_op_range(start: u32, end: u32, value: u32, l2: u32) {
    let mut i: usize;
    let mut flags: usize = 0;
    let val = value | CACHE_CLR | CACHE_OMS;
    let l2_sync: bool;

    if (end.wrapping_sub(start) >= PAGE_SIZE)
        || start < PAGE_OFFSET
        || start >= PAGE_OFFSET + LOWMEM_LIMIT
    {
        cache_op_all(value, l2);
        return;
    }

    l2_sync = (mfcr_ccr2() & CCR2_L2E) != 0 && l2 != 0;

    spin_lock_irqsave(cache_lock.as_mut_ptr(), &mut flags);
    i = (start as usize) & !(L1_CACHE_BYTES as usize - 1);
    while i < end as usize {
        cache_op_line(i, val);
        if l2_sync {
            mb();
            mtcr(b"cr24\0".as_ptr() as *const core::ffi::c_char, val);
        }
        i += L1_CACHE_BYTES as usize;
    }
    spin_unlock_irqrestore(cache_lock.as_mut_ptr(), flags);

    mb();
}

pub unsafe fn dcache_wb_line(start: usize) {
    core::arch::asm!("idly4", options(nostack, preserves_flags));
    cache_op_line(start, DATA_CACHE | CACHE_CLR);
    mb();
}

pub unsafe fn icache_inv_range(start: usize, end: usize) {
    cache_op_range(start as u32, end as u32, INS_CACHE | CACHE_INV, 0);
}

pub unsafe fn icache_inv_all() { cache_op_all(INS_CACHE | CACHE_INV, 0); }

pub unsafe fn local_icache_inv_all(_priv: *mut core::ffi::c_void) {
    cache_op_all(INS_CACHE | CACHE_INV, 0);
}

pub unsafe fn dcache_wb_range(start: usize, end: usize) {
    cache_op_range(start as u32, end as u32, DATA_CACHE | CACHE_CLR, 0);
}

pub unsafe fn dcache_wbinv_all() { cache_op_all(DATA_CACHE | CACHE_CLR | CACHE_INV, 0); }

pub unsafe fn cache_wbinv_range(start: usize, end: usize) {
    cache_op_range(start as u32, end as u32, INS_CACHE | DATA_CACHE | CACHE_CLR | CACHE_INV, 0);
}

pub unsafe fn cache_wbinv_all() {
    cache_op_all(INS_CACHE | DATA_CACHE | CACHE_CLR | CACHE_INV, 0);
}

pub unsafe fn dma_wbinv_range(start: usize, end: usize) {
    cache_op_range(start as u32, end as u32, DATA_CACHE | CACHE_CLR | CACHE_INV, 1);
}

pub unsafe fn dma_inv_range(start: usize, end: usize) {
    cache_op_range(start as u32, end as u32, DATA_CACHE | CACHE_CLR | CACHE_INV, 1);
}

pub unsafe fn dma_wb_range(start: usize, end: usize) {
    cache_op_range(start as u32, end as u32, DATA_CACHE | CACHE_CLR | CACHE_INV, 1);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
