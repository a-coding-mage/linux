// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/cpu/sh4/probe.c
 *
 * CPU Subtype Probing for SH-4.
 *
 * Copyright (C) 2001 - 2007  Paul Mundt
 * Copyright (C) 2003  Richard Curnow
 */

pub unsafe fn cpu_probe() {
    let mut pvr: ::core::ffi::c_ulong;
    let mut prr: ::core::ffi::c_ulong;
    let mut cvr: ::core::ffi::c_ulong;
    let mut size: ::core::ffi::c_ulong;

    static SIZES: [::core::ffi::c_ulong; 16] = [
        0, 1 << 12, 1 << 13, 0, 1 << 14, 0, 0, 0,
        1 << 15, 1 << 16, 0, 0, 0, 0, 0, 0,
    ];

    pvr = (__raw_readl(CCN_PVR) >> 8) & 0xffffff;
    prr = (__raw_readl(CCN_PRR) >> 4) & 0xff;
    cvr = __raw_readl(CCN_CVR);

    /* Setup some sane SH-4 defaults for the icache */
    boot_cpu_data.icache.way_incr = 1 << 13;
    boot_cpu_data.icache.entry_shift = 5;
    boot_cpu_data.icache.sets = 256;
    boot_cpu_data.icache.ways = 1;
    boot_cpu_data.icache.linesz = L1_CACHE_BYTES;

    /* And again for the dcache .. */
    boot_cpu_data.dcache.way_incr = 1 << 14;
    boot_cpu_data.dcache.entry_shift = 5;
    boot_cpu_data.dcache.sets = 512;
    boot_cpu_data.dcache.ways = 1;
    boot_cpu_data.dcache.linesz = L1_CACHE_BYTES;

    /* We don't know the chip cut */
    boot_cpu_data.cut_major = -1;
    boot_cpu_data.cut_minor = -1;

    /* Setup some generic flags we can probe on SH-4A parts */
    if (((pvr >> 16) & 0xff) == 0x10) {
        boot_cpu_data.family = CPU_FAMILY_SH4A;

        if ((cvr & 0x10000000) == 0) {
            boot_cpu_data.flags |= CPU_HAS_DSP;
            boot_cpu_data.family = CPU_FAMILY_SH4AL_DSP;
        }

        boot_cpu_data.flags |= CPU_HAS_LLSC | CPU_HAS_PERF_COUNTER;
        boot_cpu_data.cut_major = pvr & 0x7f;
        boot_cpu_data.icache.ways = 4;
        boot_cpu_data.dcache.ways = 4;
    } else {
        /* And some SH-4 defaults.. */
        boot_cpu_data.flags |= CPU_HAS_PTEA | CPU_HAS_FPU;
        boot_cpu_data.family = CPU_FAMILY_SH4;
    }

    /* FPU detection works for almost everyone */
    if (cvr & 0x20000000) {
        boot_cpu_data.flags |= CPU_HAS_FPU;
    }

    /* Mask off the upper chip ID */
    pvr &= 0xffff;

    /* Probe the underlying processor version/revision and adjust cpu_data setup accordingly. */
    match pvr {
        0x205 => { boot_cpu_data.type = CPU_SH7750; boot_cpu_data.flags |= CPU_HAS_P2_FLUSH_BUG | CPU_HAS_PERF_COUNTER; }
        0x206 => { boot_cpu_data.type = CPU_SH7750S; boot_cpu_data.flags |= CPU_HAS_P2_FLUSH_BUG | CPU_HAS_PERF_COUNTER; }
        0x1100 => boot_cpu_data.type = CPU_SH7751,
        0x2001 | 0x2004 => boot_cpu_data.type = CPU_SH7770,
        0x2006 | 0x200a => {
            if prr == 0x61 { boot_cpu_data.type = CPU_SH7781; }
            else if prr == 0xa1 { boot_cpu_data.type = CPU_SH7763; }
            else { boot_cpu_data.type = CPU_SH7780; }
        }
        0x3000 | 0x3003 | 0x3009 => boot_cpu_data.type = CPU_SH7343,
        0x3004 | 0x3007 => boot_cpu_data.type = CPU_SH7785,
        0x4004 | 0x4005 => { boot_cpu_data.type = CPU_SH7786; boot_cpu_data.flags |= CPU_HAS_PTEAEX | CPU_HAS_L2_CACHE; }
        0x3008 => match prr {
            0x50 | 0x51 => { boot_cpu_data.type = CPU_SH7723; boot_cpu_data.flags |= CPU_HAS_L2_CACHE; }
            0x70 => boot_cpu_data.type = CPU_SH7366,
            0xa0 | 0xa1 => boot_cpu_data.type = CPU_SH7722,
            _ => {}
        },
        0x300b => match prr {
            0x20 => { boot_cpu_data.type = CPU_SH7724; boot_cpu_data.flags |= CPU_HAS_L2_CACHE; }
            0x10 | 0x11 => boot_cpu_data.type = CPU_SH7757,
            0xd0 | 0x40 => boot_cpu_data.type = CPU_SH7372, // yon-ten-go
            0xe0 => boot_cpu_data.type = CPU_SH7734, // SH7733/SH7734
            _ => {}
        },
        0x4000 | 0x4001 => boot_cpu_data.type = CPU_SHX3,
        0x700 => { boot_cpu_data.type = CPU_SH4_501; boot_cpu_data.flags &= !CPU_HAS_FPU; boot_cpu_data.icache.ways = 2; boot_cpu_data.dcache.ways = 2; }
        0x600 => { boot_cpu_data.type = CPU_SH4_202; boot_cpu_data.icache.ways = 2; boot_cpu_data.dcache.ways = 2; }
        0x500..=0x501 => {
            match prr {
                0x10 => boot_cpu_data.type = CPU_SH7750R,
                0x11 => boot_cpu_data.type = CPU_SH7751R,
                0x50..=0x5f => boot_cpu_data.type = CPU_SH7760,
                _ => {}
            }
            boot_cpu_data.icache.ways = 2;
            boot_cpu_data.dcache.ways = 2;
        }
        _ => {}
    }

    /* On anything that's not a direct-mapped cache, look to the CVR for I/D-cache specifics. */
    if boot_cpu_data.icache.ways > 1 {
        size = SIZES[((cvr >> 20) & 0xf) as usize];
        boot_cpu_data.icache.way_incr = size >> 1;
        boot_cpu_data.icache.sets = size >> 6;
    }

    /* And the rest of the D-cache */
    if boot_cpu_data.dcache.ways > 1 {
        size = SIZES[((cvr >> 16) & 0xf) as usize];
        boot_cpu_data.dcache.way_incr = size >> 1;
        boot_cpu_data.dcache.sets = size >> 6;
    }

    /* SH-4A's have an optional PIPT L2. */
    if boot_cpu_data.flags & CPU_HAS_L2_CACHE != 0 {
        /* Verify that it really has something hooked up. */
        if cvr & 0xf == 0 {
            boot_cpu_data.flags &= !CPU_HAS_L2_CACHE;
        } else {
            /* Silicon and specifications have clearly never met.. */
            cvr ^= 0xf;
            /* Sizes are 128KB, 256KB, 512KB, and 1MB. */
            size = (cvr & 0xf) << 17;
            boot_cpu_data.scache.way_incr = 1 << 16;
            boot_cpu_data.scache.entry_shift = 5;
            boot_cpu_data.scache.ways = 4;
            boot_cpu_data.scache.linesz = L1_CACHE_BYTES;
            boot_cpu_data.scache.entry_mask = boot_cpu_data.scache.way_incr - boot_cpu_data.scache.linesz;
            boot_cpu_data.scache.sets = size / (boot_cpu_data.scache.linesz * boot_cpu_data.scache.ways);
            boot_cpu_data.scache.way_size = boot_cpu_data.scache.sets * boot_cpu_data.scache.linesz;
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
