// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/cpu/sh2a/probe.c
 *
 * CPU Subtype Probing for SH-2A.
 *
 * Copyright (C) 2004 - 2007  Paul Mundt
 */

// C dependencies: linux/init.h, asm/processor.h, and asm/cache.h.

pub unsafe fn cpu_probe() {
    boot_cpu_data.family = CPU_FAMILY_SH2A;

    /* All SH-2A CPUs have support for 16 and 32-bit opcodes.. */
    boot_cpu_data.flags |= CPU_HAS_OP32;

    // These build-time CONFIG_CPU_SUBTYPE_* conditions are preserved as Rust cfgs.
    #[cfg(CONFIG_CPU_SUBTYPE_SH7201)]
    {
        boot_cpu_data.type_ = CPU_SH7201;
        boot_cpu_data.flags |= CPU_HAS_FPU;
    }
    #[cfg(CONFIG_CPU_SUBTYPE_SH7203)]
    {
        boot_cpu_data.type_ = CPU_SH7203;
        boot_cpu_data.flags |= CPU_HAS_FPU;
    }
    #[cfg(CONFIG_CPU_SUBTYPE_SH7263)]
    {
        boot_cpu_data.type_ = CPU_SH7263;
        boot_cpu_data.flags |= CPU_HAS_FPU;
    }
    #[cfg(CONFIG_CPU_SUBTYPE_SH7264)]
    {
        boot_cpu_data.type_ = CPU_SH7264;
        boot_cpu_data.flags |= CPU_HAS_FPU;
    }
    #[cfg(CONFIG_CPU_SUBTYPE_SH7269)]
    {
        boot_cpu_data.type_ = CPU_SH7269;
        boot_cpu_data.flags |= CPU_HAS_FPU;
    }
    #[cfg(CONFIG_CPU_SUBTYPE_SH7206)]
    {
        boot_cpu_data.type_ = CPU_SH7206;
        boot_cpu_data.flags |= CPU_HAS_DSP;
    }
    #[cfg(CONFIG_CPU_SUBTYPE_MXG)]
    {
        boot_cpu_data.type_ = CPU_MXG;
        boot_cpu_data.flags |= CPU_HAS_DSP;
    }

    boot_cpu_data.dcache.ways = 4;
    boot_cpu_data.dcache.way_incr = 1 << 11;
    boot_cpu_data.dcache.sets = 128;
    boot_cpu_data.dcache.entry_shift = 4;
    boot_cpu_data.dcache.linesz = L1_CACHE_BYTES;
    boot_cpu_data.dcache.flags = 0;

    /*
     * The icache is the same as the dcache as far as this setup is
     * concerned. The only real difference in hardware is that the icache
     * lacks the U bit that the dcache has, none of this has any bearing
     * on the cache info.
     */
    boot_cpu_data.icache = boot_cpu_data.dcache;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
