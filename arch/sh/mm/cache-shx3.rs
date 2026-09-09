/*
 * arch/sh/mm/cache-shx3.c - SH-X3 optimized cache ops
 *
 * Copyright (C) 2010  Paul Mundt
 *
 * This file is subject to the terms and conditions of the GNU General
 * Public License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Dependencies supplied by the surrounding kernel translation.

const CCR_CACHE_SNM: u32 = 0x40000; // Hardware-assisted synonym avoidance
const CCR_CACHE_IBE: u32 = 0x1000000; // ICBI broadcast

#[repr(C)]
pub struct CacheInfo {
    pub n_aliases: u32,
}

#[repr(C)]
pub struct CpuData {
    pub dcache: CacheInfo,
    pub icache: CacheInfo,
}

extern "C" {
    static mut boot_cpu_data: CpuData;
    static SH_CCR: *mut core::ffi::c_void;

    fn __raw_readl(addr: *mut core::ffi::c_void) -> u32;
    fn writel_uncached(value: u32, addr: *mut core::ffi::c_void);
    fn pr_info(fmt: *const core::ffi::c_char, ...);
}

pub unsafe fn shx3_cache_init() {
    let mut ccr: u32;

    ccr = __raw_readl(SH_CCR);

    /*
     * If we've got cache aliases, resolve them in hardware.
     */
    if (*core::ptr::addr_of!(boot_cpu_data)).dcache.n_aliases != 0
        || (*core::ptr::addr_of!(boot_cpu_data)).icache.n_aliases != 0
    {
        ccr |= CCR_CACHE_SNM;

        (*core::ptr::addr_of_mut!(boot_cpu_data)).icache.n_aliases = 0;
        (*core::ptr::addr_of_mut!(boot_cpu_data)).dcache.n_aliases = 0;

        pr_info(b"Enabling hardware synonym avoidance\0".as_ptr() as *const core::ffi::c_char);
    }

    // CONFIG_SMP: Broadcast I-cache block invalidations by default.
    // The guarded C code is retained here when SMP support is enabled.
    #[cfg(CONFIG_SMP)]
    {
        ccr |= CCR_CACHE_IBE;
    }

    writel_uncached(ccr, SH_CCR);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
