// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2006 Chris Dearman (chris@mips.com),
 */

/* MIPS32/MIPS64 L2 cache handling */

/*
 * Writeback and invalidate the secondary cache before DMA.
 */
unsafe fn mips_sc_wback_inv(addr: c_ulong, size: c_ulong) {
    blast_scache_range(addr, addr.wrapping_add(size));
}

/*
 * Invalidate the secondary cache before DMA.
 */
unsafe fn mips_sc_inv(addr: c_ulong, size: c_ulong) {
    let lsize: c_ulong = cpu_scache_line_size();
    let almask: c_ulong = !(lsize.wrapping_sub(1));

    cache_op(Hit_Writeback_Inv_SD, addr & almask);
    cache_op(
        Hit_Writeback_Inv_SD,
        addr.wrapping_add(size).wrapping_sub(1) & almask,
    );
    blast_inv_scache_range(addr, addr.wrapping_add(size));
}

unsafe fn mips_sc_enable() {
    /* L2 cache is permanently enabled */
}

unsafe fn mips_sc_disable() {
    /* L2 cache is permanently enabled */
}

unsafe fn mips_sc_prefetch_enable() {
    let mut pftctl: c_ulong;

    if mips_cm_revision() < CM_REV_CM2_5 {
        return;
    }

    /*
     * If there is one or more L2 prefetch unit present then enable
     * prefetching for both code & data, for all ports.
     */
    pftctl = read_gcr_l2_pft_control();
    if pftctl & CM_GCR_L2_PFT_CONTROL_NPFT != 0 {
        pftctl &= !CM_GCR_L2_PFT_CONTROL_PAGEMASK;
        pftctl |= PAGE_MASK & CM_GCR_L2_PFT_CONTROL_PAGEMASK;
        pftctl |= CM_GCR_L2_PFT_CONTROL_PFTEN;
        write_gcr_l2_pft_control(pftctl);

        set_gcr_l2_pft_control_b(
            CM_GCR_L2_PFT_CONTROL_B_PORTID | CM_GCR_L2_PFT_CONTROL_B_CEN,
        );
    }
}

unsafe fn mips_sc_prefetch_disable() {
    if mips_cm_revision() < CM_REV_CM2_5 {
        return;
    }

    clear_gcr_l2_pft_control(CM_GCR_L2_PFT_CONTROL_PFTEN);
    clear_gcr_l2_pft_control_b(
        CM_GCR_L2_PFT_CONTROL_B_PORTID | CM_GCR_L2_PFT_CONTROL_B_CEN,
    );
}

unsafe fn mips_sc_prefetch_is_enabled() -> bool {
    let pftctl: c_ulong;

    if mips_cm_revision() < CM_REV_CM2_5 {
        return false;
    }

    pftctl = read_gcr_l2_pft_control();
    if pftctl & CM_GCR_L2_PFT_CONTROL_NPFT == 0 {
        return false;
    }
    (pftctl & CM_GCR_L2_PFT_CONTROL_PFTEN) != 0
}

static mut mips_sc_ops: bcache_ops = bcache_ops {
    bc_enable: Some(mips_sc_enable),
    bc_disable: Some(mips_sc_disable),
    bc_wback_inv: Some(mips_sc_wback_inv),
    bc_inv: Some(mips_sc_inv),
    bc_prefetch_enable: Some(mips_sc_prefetch_enable),
    bc_prefetch_disable: Some(mips_sc_prefetch_disable),
    bc_prefetch_is_enabled: Some(mips_sc_prefetch_is_enabled),
};

/*
 * Check if the L2 cache controller is activated on a particular platform.
 * MTI's L2 controller and the L2 cache controller of Broadcom's BMIPS
 * cores both use c0_config2's bit 12 as "L2 Bypass" bit, that is the
 * cache being disabled.  However there is no guarantee for this to be
 * true on all platforms.  In an act of stupidity the spec defined bits
 * 12..15 as implementation defined so below function will eventually have
 * to be replaced by a platform specific probe.
 */
unsafe fn mips_sc_is_activated(c: *mut cpuinfo_mips) -> c_int {
    let config2: c_uint = read_c0_config2();
    let tmp: c_uint;

    /* Check the bypass bit (L2B) */
    match current_cpu_type() {
        CPU_34K | CPU_74K | CPU_1004K | CPU_1074K | CPU_INTERAPTIV |
        CPU_PROAPTIV | CPU_P5600 | CPU_BMIPS5000 | CPU_QEMU_GENERIC |
        CPU_P6600 => {
            if config2 & (1 << 12) != 0 {
                return 0;
            }
        }
        _ => {}
    }

    tmp = (config2 >> 4) & 0x0f;
    if 0 < tmp && tmp <= 7 {
        (*c).scache.linesz = 2 << tmp;
    } else {
        return 0;
    }
    1
}

unsafe fn mips_sc_probe_cm3() -> c_int {
    let c: *mut cpuinfo_mips = &mut current_cpu_data;
    let cfg: c_ulong = read_gcr_l2_config();
    let mut sets: c_ulong;
    let mut line_sz: c_ulong;
    let mut assoc: c_ulong;

    if cfg & CM_GCR_L2_CONFIG_BYPASS != 0 {
        return 0;
    }

    sets = cfg & CM_GCR_L2_CONFIG_SET_SIZE;
    sets >>= __ffs(CM_GCR_L2_CONFIG_SET_SIZE);
    if sets != 0 {
        (*c).scache.sets = 64 << sets;
    }

    line_sz = cfg & CM_GCR_L2_CONFIG_LINE_SIZE;
    line_sz >>= __ffs(CM_GCR_L2_CONFIG_LINE_SIZE);
    if line_sz != 0 {
        (*c).scache.linesz = 2 << line_sz;
    }

    assoc = cfg & CM_GCR_L2_CONFIG_ASSOC;
    assoc >>= __ffs(CM_GCR_L2_CONFIG_ASSOC);
    (*c).scache.ways = assoc + 1;
    (*c).scache.waysize = (*c).scache.sets * (*c).scache.linesz;
    (*c).scache.waybit = __ffs((*c).scache.waysize);

    if (*c).scache.linesz != 0 {
        (*c).scache.flags &= !MIPS_CACHE_NOT_PRESENT;
        (*c).options |= MIPS_CPU_INCLUSIVE_CACHES;
        return 1;
    }

    0
}

unsafe fn mips_sc_probe() -> c_int {
    let c: *mut cpuinfo_mips = &mut current_cpu_data;
    let config1: c_uint;
    let config2: c_uint;
    let mut tmp: c_uint;

    /* Mark as not present until probe completed */
    (*c).scache.flags |= MIPS_CACHE_NOT_PRESENT;

    if mips_cm_revision() >= CM_REV_CM3 {
        return mips_sc_probe_cm3();
    }

    /* Ignore anything but MIPSxx processors */
    if (*c).isa_level & (MIPS_CPU_ISA_M32R1 | MIPS_CPU_ISA_M64R1 |
        MIPS_CPU_ISA_M32R2 | MIPS_CPU_ISA_M64R2 |
        MIPS_CPU_ISA_M32R5 | MIPS_CPU_ISA_M64R5 |
        MIPS_CPU_ISA_M32R6 | MIPS_CPU_ISA_M64R6) == 0 {
        return 0;
    }

    /* Does this MIPS32/MIPS64 CPU have a config2 register? */
    config1 = read_c0_config1();
    if config1 & MIPS_CONF_M == 0 {
        return 0;
    }

    config2 = read_c0_config2();

    if mips_sc_is_activated(c) == 0 {
        return 0;
    }

    tmp = (config2 >> 8) & 0x0f;
    if tmp <= 7 {
        (*c).scache.sets = 64 << tmp;
    } else {
        return 0;
    }

    tmp = config2 & 0x0f;
    if tmp <= 7 {
        (*c).scache.ways = tmp + 1;
    } else {
        return 0;
    }

    if current_cpu_type() == CPU_XBURST {
        match mips_machtype {
            /*
             * According to config2 it would be 5-ways, but that is
             * contradicted by all documentation.
             */
            MACH_INGENIC_JZ4770 | MACH_INGENIC_JZ4775 => {
                (*c).scache.ways = 4;
            }

            /*
             * According to config2 it would be 5-ways and 512-sets,
             * but that is contradicted by all documentation.
             */
            MACH_INGENIC_X1000 | MACH_INGENIC_X1000E => {
                (*c).scache.sets = 256;
                (*c).scache.ways = 4;
            }
            _ => {}
        }
    }

    (*c).scache.waysize = (*c).scache.sets * (*c).scache.linesz;
    (*c).scache.waybit = __ffs((*c).scache.waysize);

    (*c).scache.flags &= !MIPS_CACHE_NOT_PRESENT;

    1
}

pub unsafe fn mips_sc_init() -> c_int {
    let found: c_int = mips_sc_probe();
    if found != 0 {
        mips_sc_enable();
        mips_sc_prefetch_enable();
        bcops = &mut mips_sc_ops;
    }
    found
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
