// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/sh/mm/cache-j2.c
 *
 * Copyright (C) 2015-2016 Smart Energy Instruments, Inc.
 */

// Linux kernel headers supplied by the surrounding translation unit:
// linux/init.h, linux/mm.h, linux/cpumask.h, asm/cache.h,
// asm/addrspace.h, asm/processor.h, asm/cacheflush.h, and asm/io.h.

use core::ffi::c_void;

const ICACHE_ENABLE: u32 = 0x1;
const DCACHE_ENABLE: u32 = 0x2;
const CACHE_ENABLE: u32 = ICACHE_ENABLE | DCACHE_ENABLE;
const ICACHE_FLUSH: u32 = 0x100;
const DCACHE_FLUSH: u32 = 0x200;
const CACHE_FLUSH: u32 = ICACHE_FLUSH | DCACHE_FLUSH;

pub static mut j2_ccr_base: *mut u32 = core::ptr::null_mut();

// The following declarations are provided by the surrounding kernel code.
type CacheFlushFn = unsafe extern "C" fn(*mut c_void);
extern "C" {
    static mut local_flush_cache_all: Option<CacheFlushFn>;
    static mut local_flush_cache_mm: Option<CacheFlushFn>;
    static mut local_flush_cache_dup_mm: Option<CacheFlushFn>;
    static mut local_flush_cache_page: Option<CacheFlushFn>;
    static mut local_flush_cache_range: Option<CacheFlushFn>;
    static mut local_flush_dcache_folio: Option<CacheFlushFn>;
    static mut local_flush_icache_range: Option<CacheFlushFn>;
    static mut local_flush_icache_folio: Option<CacheFlushFn>;
    static mut local_flush_cache_sigtramp: Option<CacheFlushFn>;
    static nr_cpu_ids: usize;
    fn __raw_writel(value: u32, address: *mut u32);
    fn __raw_readl(address: *mut u32) -> u32;
    fn pr_info(format: *const u8, ...);
}

unsafe extern "C" fn j2_flush_icache(_args: *mut c_void) {
    // `for_each_possible_cpu(cpu)` expands to iteration over possible CPUs.
    for cpu in 0..nr_cpu_ids {
        __raw_writel(CACHE_ENABLE | ICACHE_FLUSH, j2_ccr_base.add(cpu));
    }
}

unsafe extern "C" fn j2_flush_dcache(_args: *mut c_void) {
    // `for_each_possible_cpu(cpu)` expands to iteration over possible CPUs.
    for cpu in 0..nr_cpu_ids {
        __raw_writel(CACHE_ENABLE | DCACHE_FLUSH, j2_ccr_base.add(cpu));
    }
}

unsafe extern "C" fn j2_flush_both(_args: *mut c_void) {
    // `for_each_possible_cpu(cpu)` expands to iteration over possible CPUs.
    for cpu in 0..nr_cpu_ids {
        __raw_writel(CACHE_ENABLE | CACHE_FLUSH, j2_ccr_base.add(cpu));
    }
}

// __init
pub unsafe extern "C" fn j2_cache_init() {
    if j2_ccr_base.is_null() {
        return;
    }

    local_flush_cache_all = Some(j2_flush_both);
    local_flush_cache_mm = Some(j2_flush_both);
    local_flush_cache_dup_mm = Some(j2_flush_both);
    local_flush_cache_page = Some(j2_flush_both);
    local_flush_cache_range = Some(j2_flush_both);
    local_flush_dcache_folio = Some(j2_flush_dcache);
    local_flush_icache_range = Some(j2_flush_icache);
    local_flush_icache_folio = Some(j2_flush_icache);
    local_flush_cache_sigtramp = Some(j2_flush_icache);

    pr_info(b"Initial J2 CCR is %.8x\0".as_ptr(), __raw_readl(j2_ccr_base));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
