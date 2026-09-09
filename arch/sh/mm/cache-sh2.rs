// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/sh/mm/cache-sh2.c
 *
 * Copyright (C) 2002 Paul Mundt
 * Copyright (C) 2008 Yoshinori Sato
 */

use core::ffi::c_void;

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    static mut __flush_wback_region:
        Option<unsafe extern "C" fn(start: *mut c_void, size: i32)>;
    static mut __flush_purge_region:
        Option<unsafe extern "C" fn(start: *mut c_void, size: i32)>;
    static mut __flush_invalidate_region:
        Option<unsafe extern "C" fn(start: *mut c_void, size: i32)>;

    fn __raw_readl(addr: u64) -> u32;
    fn __raw_writel(value: u32, addr: u64);
    fn local_irq_save(flags: *mut u64);
    fn local_irq_restore(flags: u64);
    fn jump_to_uncached();
    fn back_to_cached();
}

unsafe extern "C" fn sh2__flush_wback_region(start: *mut c_void, size: i32) {
    let mut v: u64;
    let begin: u64 = start as u64 & !(L1_CACHE_BYTES - 1);
    let end: u64 = (start as u64 + size as u64 + L1_CACHE_BYTES - 1)
        & !(L1_CACHE_BYTES - 1);
    v = begin;
    while v < end {
        let addr = CACHE_OC_ADDRESS_ARRAY | (v & 0x00000ff0);
        let mut way: i32 = 0;
        while way < 4 {
            let mut data = __raw_readl(addr | ((way as u64) << 12));
            if (data as u64 & CACHE_PHYSADDR_MASK) == (v & CACHE_PHYSADDR_MASK) {
                data &= !(SH_CACHE_UPDATED as u32);
                __raw_writel(data, addr | ((way as u64) << 12));
            }
            way += 1;
        }
        v += L1_CACHE_BYTES;
    }
}

unsafe extern "C" fn sh2__flush_purge_region(start: *mut c_void, size: i32) {
    let begin: u64 = start as u64 & !(L1_CACHE_BYTES - 1);
    let end: u64 = (start as u64 + size as u64 + L1_CACHE_BYTES - 1)
        & !(L1_CACHE_BYTES - 1);
    let mut v = begin;
    while v < end {
        __raw_writel(
            (v & CACHE_PHYSADDR_MASK) as u32,
            CACHE_OC_ADDRESS_ARRAY | (v & 0x00000ff0) | 0x00000008,
        );
        v += L1_CACHE_BYTES;
    }
}

unsafe extern "C" fn sh2__flush_invalidate_region(start: *mut c_void, size: i32) {
    #[cfg(CONFIG_CACHE_WRITEBACK)]
    {
        // SH-2 does not support individual line invalidation, only a
        // global invalidate.
        let mut ccr: u32;
        let mut flags: u64 = 0;
        local_irq_save(&mut flags);
        jump_to_uncached();

        ccr = __raw_readl(SH_CCR);
        ccr |= CCR_CACHE_INVALIDATE;
        __raw_writel(ccr, SH_CCR);

        back_to_cached();
        local_irq_restore(flags);
    }
    #[cfg(not(CONFIG_CACHE_WRITEBACK))]
    {
        let begin: u64 = start as u64 & !(L1_CACHE_BYTES - 1);
        let end: u64 = (start as u64 + size as u64 + L1_CACHE_BYTES - 1)
            & !(L1_CACHE_BYTES - 1);
        let mut v = begin;
        while v < end {
            __raw_writel(
                (v & CACHE_PHYSADDR_MASK) as u32,
                CACHE_OC_ADDRESS_ARRAY | (v & 0x00000ff0) | 0x00000008,
            );
            v += L1_CACHE_BYTES;
        }
    }
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn sh2_cache_init() {
    __flush_wback_region = Some(sh2__flush_wback_region);
    __flush_purge_region = Some(sh2__flush_purge_region);
    __flush_invalidate_region = Some(sh2__flush_invalidate_region);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
