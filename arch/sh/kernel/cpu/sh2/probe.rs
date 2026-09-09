// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/cpu/sh2/probe.c
 *
 * CPU Subtype Probing for SH-2.
 *
 * Copyright (C) 2002 Paul Mundt
 */

// Linux and architecture headers from the original source provide these
// declarations and constants in the surrounding translation unit.

#[cfg(CONFIG_CPU_J2)]
extern "C" {
    static mut j2_ccr_base: *mut u32;
}

#[cfg(CONFIG_CPU_J2)]
unsafe fn scan_cache(
    node: usize,
    _uname: *const core::ffi::c_char,
    _depth: i32,
    _data: *mut core::ffi::c_void,
) -> i32 {
    if !of_flat_dt_is_compatible(node, b"jcore,cache\0".as_ptr() as *const core::ffi::c_char) {
        return 0;
    }

    j2_ccr_base = ioremap(of_flat_dt_translate_address(node), 4);

    1
}

pub unsafe fn cpu_probe() {
    #[cfg(CONFIG_CPU_SUBTYPE_SH7619)]
    {
        boot_cpu_data.type_ = CPU_SH7619;
        boot_cpu_data.dcache.ways = 4;
        boot_cpu_data.dcache.way_incr = 1 << 12;
        boot_cpu_data.dcache.sets = 256;
        boot_cpu_data.dcache.entry_shift = 4;
        boot_cpu_data.dcache.linesz = L1_CACHE_BYTES;
        boot_cpu_data.dcache.flags = 0;
    }

    #[cfg(CONFIG_CPU_J2)]
    {
        #[cfg(CONFIG_SMP)]
        let cpu: u32 = hard_smp_processor_id();
        #[cfg(not(CONFIG_SMP))]
        let cpu: u32 = 0;

        if cpu == 0 {
            of_scan_flat_dt(scan_cache, core::ptr::null_mut());
        }
        if !j2_ccr_base.is_null() {
            __raw_writel(0x80000303, j2_ccr_base.add((4 * cpu) as usize));
        }
        if cpu != 0 {
            return;
        }
        boot_cpu_data.type_ = CPU_J2;

        /* These defaults are appropriate for the original/current
         * J2 cache. Once there is a proper framework for getting cache
         * info from device tree, we should switch to that. */
        boot_cpu_data.dcache.ways = 1;
        boot_cpu_data.dcache.sets = 256;
        boot_cpu_data.dcache.entry_shift = 5;
        boot_cpu_data.dcache.linesz = 32;
        boot_cpu_data.dcache.flags = 0;

        boot_cpu_data.flags |= CPU_HAS_CAS_L;
    }

    #[cfg(not(CONFIG_CPU_J2))]
    {
        /*
         * SH-2 doesn't have separate caches
         */
        boot_cpu_data.dcache.flags |= SH_CACHE_COMBINED;
    }
    boot_cpu_data.icache = boot_cpu_data.dcache;
    boot_cpu_data.family = CPU_FAMILY_SH2;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
