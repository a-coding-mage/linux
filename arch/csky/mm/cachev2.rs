// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.

/* Dependencies supplied by the surrounding kernel translation unit. */
extern "C" {
    fn mtcr(reg: *const core::ffi::c_char, value: u32);
    fn sync_is();
    fn irqs_disabled() -> bool;
    fn on_each_cpu(func: unsafe extern "C" fn(*mut core::ffi::c_void), info: *mut core::ffi::c_void, wait: i32);
}

/* for L1-cache */
const INS_CACHE: u32 = 1 << 0;
const DATA_CACHE: u32 = 1 << 1;
const CACHE_INV: u32 = 1 << 4;
const CACHE_CLR: u32 = 1 << 5;
const CACHE_OMS: u32 = 1 << 6;

/* Supplied by asm/cache.h. */
extern "C" {
    static L1_CACHE_BYTES: usize;
}

pub unsafe extern "C" fn local_icache_inv_all(_priv: *mut core::ffi::c_void) {
    mtcr(b"cr17\0".as_ptr() as *const core::ffi::c_char, INS_CACHE | CACHE_INV);
    sync_is();
}

#[cfg(feature = "CONFIG_CPU_HAS_ICACHE_INS")]
pub unsafe extern "C" fn icache_inv_range(start: usize, end: usize) {
    let mut i = start & !(L1_CACHE_BYTES - 1);
    while i < end {
        core::arch::asm!("icache.iva {0}", in(reg) i, options(nostack, preserves_flags));
        i = i.wrapping_add(L1_CACHE_BYTES);
    }
    sync_is();
}

#[cfg(not(feature = "CONFIG_CPU_HAS_ICACHE_INS"))]
#[repr(C)]
struct CacheRange {
    start: usize,
    end: usize,
}

#[cfg(not(feature = "CONFIG_CPU_HAS_ICACHE_INS"))]
#[repr(C)]
struct CacheLock {
    _opaque: [u8; 0],
}

#[cfg(not(feature = "CONFIG_CPU_HAS_ICACHE_INS"))]
extern "C" {
    static mut cache_lock: CacheLock;
    fn spin_lock_irqsave(lock: *mut CacheLock, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut CacheLock, flags: usize);
}

#[cfg(not(feature = "CONFIG_CPU_HAS_ICACHE_INS"))]
#[inline]
unsafe fn cache_op_line(i: usize, val: u32) {
    mtcr(b"cr22\0".as_ptr() as *const core::ffi::c_char, i as u32);
    mtcr(b"cr17\0".as_ptr() as *const core::ffi::c_char, val);
}

#[cfg(not(feature = "CONFIG_CPU_HAS_ICACHE_INS"))]
pub unsafe extern "C" fn local_icache_inv_range(priv_: *mut core::ffi::c_void) {
    let param = &*(priv_ as *const CacheRange);
    let mut i = param.start & !(L1_CACHE_BYTES - 1);
    let mut flags: usize = 0;

    spin_lock_irqsave(&mut cache_lock, &mut flags);
    while i < param.end {
        cache_op_line(i, INS_CACHE | CACHE_INV | CACHE_OMS);
        i = i.wrapping_add(L1_CACHE_BYTES);
    }
    spin_unlock_irqrestore(&mut cache_lock, flags);
    sync_is();
}

#[cfg(not(feature = "CONFIG_CPU_HAS_ICACHE_INS"))]
pub unsafe extern "C" fn icache_inv_range(start: usize, end: usize) {
    let mut param = CacheRange { start, end };
    if irqs_disabled() {
        local_icache_inv_range(&mut param as *mut _ as *mut core::ffi::c_void);
    } else {
        on_each_cpu(local_icache_inv_range, &mut param as *mut _ as *mut core::ffi::c_void, 1);
    }
}

#[inline]
pub unsafe extern "C" fn dcache_wb_line(start: usize) {
    core::arch::asm!("dcache.cval1 {0}", in(reg) start, options(nostack, preserves_flags));
    sync_is();
}

pub unsafe extern "C" fn dcache_wb_range(start: usize, end: usize) {
    let mut i = start & !(L1_CACHE_BYTES - 1);
    while i < end {
        core::arch::asm!("dcache.cval1 {0}", in(reg) i, options(nostack, preserves_flags));
        i = i.wrapping_add(L1_CACHE_BYTES);
    }
    sync_is();
}

pub unsafe extern "C" fn cache_wbinv_range(start: usize, end: usize) {
    dcache_wb_range(start, end);
    icache_inv_range(start, end);
}

pub unsafe extern "C" fn dma_wbinv_range(start: usize, end: usize) {
    let mut i = start & !(L1_CACHE_BYTES - 1);
    while i < end {
        core::arch::asm!("dcache.civa {0}", in(reg) i, options(nostack, preserves_flags));
        i = i.wrapping_add(L1_CACHE_BYTES);
    }
    sync_is();
}

pub unsafe extern "C" fn dma_inv_range(start: usize, end: usize) {
    let mut i = start & !(L1_CACHE_BYTES - 1);
    while i < end {
        core::arch::asm!("dcache.iva {0}", in(reg) i, options(nostack, preserves_flags));
        i = i.wrapping_add(L1_CACHE_BYTES);
    }
    sync_is();
}

pub unsafe extern "C" fn dma_wb_range(start: usize, end: usize) {
    let mut i = start & !(L1_CACHE_BYTES - 1);
    while i < end {
        core::arch::asm!("dcache.cva {0}", in(reg) i, options(nostack, preserves_flags));
        i = i.wrapping_add(L1_CACHE_BYTES);
    }
    sync_is();
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
