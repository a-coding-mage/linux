// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/sh/mm/cache-sh3.c
 *
 * Copyright (C) 1999, 2000  Niibe Yutaka
 * Copyright (C) 2002 Paul Mundt
 */

// The declarations below are supplied by the surrounding kernel translation.

#[repr(C)]
pub struct Sh3Dcache {
    pub ways: usize,
    pub entry_mask: usize,
    pub way_incr: usize,
}

#[repr(C)]
pub struct Sh3CpuData {
    pub dcache: Sh3Dcache,
}

extern "C" {
    static mut current_cpu_data: Sh3CpuData;

    static mut __flush_wback_region: Option<unsafe extern "C" fn(*mut core::ffi::c_void, i32)>;
    static mut __flush_purge_region: Option<unsafe extern "C" fn(*mut core::ffi::c_void, i32)>;
    static mut __flush_invalidate_region: Option<unsafe extern "C" fn(*mut core::ffi::c_void, i32)>;

    fn __pa(addr: usize) -> usize;
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn __raw_readl(addr: usize) -> u32;
    fn __raw_writel(value: u32, addr: usize);
}

extern "C" {
    static CACHE_OC_ADDRESS_ARRAY: usize;
    static L1_CACHE_BYTES: usize;
    static CACHE_PHYSADDR_MASK: u32;
    static SH_CACHE_UPDATED: u32;
    static SH_CACHE_ASSOC: usize;
}

/*
 * Write back the dirty D-caches, but not invalidate them.
 *
 * Is this really worth it, or should we just alias this routine
 * to __flush_purge_region too?
 *
 * START: Virtual Address (U0, P1, or P3)
 * SIZE: Size of the region.
 */
unsafe extern "C" fn sh3__flush_wback_region(start: *mut core::ffi::c_void, size: i32) {
    let mut v: usize;
    let mut j: usize;
    let begin: usize = start as usize & !(L1_CACHE_BYTES - 1);
    let end: usize = (start as usize + size as usize + L1_CACHE_BYTES - 1)
        & !(L1_CACHE_BYTES - 1);
    let mut flags: usize = 0;

    v = begin;
    while v < end {
        let mut addrstart: usize = CACHE_OC_ADDRESS_ARRAY;
        j = 0;
        while j < current_cpu_data.dcache.ways {
            let p: usize = __pa(v);
            let addr: usize = addrstart | (v & current_cpu_data.dcache.entry_mask);
            local_irq_save(&mut flags as *mut usize);
            let mut data: u32 = __raw_readl(addr);

            if (data & CACHE_PHYSADDR_MASK) == ((p as u32) & CACHE_PHYSADDR_MASK) {
                data &= !SH_CACHE_UPDATED;
                __raw_writel(data, addr);
                local_irq_restore(flags);
                break;
            }
            local_irq_restore(flags);
            addrstart += current_cpu_data.dcache.way_incr;
            j += 1;
        }
        v += L1_CACHE_BYTES;
    }
}

/*
 * Write back the dirty D-caches and invalidate them.
 *
 * START: Virtual Address (U0, P1, or P3)
 * SIZE: Size of the region.
 */
unsafe extern "C" fn sh3__flush_purge_region(start: *mut core::ffi::c_void, size: i32) {
    let begin: usize = start as usize & !(L1_CACHE_BYTES - 1);
    let end: usize = (start as usize + size as usize + L1_CACHE_BYTES - 1)
        & !(L1_CACHE_BYTES - 1);
    let mut v: usize = begin;

    while v < end {
        let data: usize = v & 0xfffffc00; /* _Virtual_ address, ~U, ~V */
        let addr: usize = CACHE_OC_ADDRESS_ARRAY
            | (v & current_cpu_data.dcache.entry_mask)
            | SH_CACHE_ASSOC;
        __raw_writel(data as u32, addr);
        v += L1_CACHE_BYTES;
    }
}

pub unsafe extern "C" fn sh3_cache_init() {
    __flush_wback_region = Some(sh3__flush_wback_region);
    __flush_purge_region = Some(sh3__flush_purge_region);

    /*
     * No write back please
     *
     * Except I don't think there's any way to avoid the writeback.
     * So we just alias it to sh3__flush_purge_region(). dwmw2.
     */
    __flush_invalidate_region = Some(sh3__flush_purge_region);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
