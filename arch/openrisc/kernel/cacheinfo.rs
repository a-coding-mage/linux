// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * OpenRISC cacheinfo support
 *
 * Based on work done for MIPS and LoongArch. All original copyrights
 * apply as per the original source declaration.
 *
 * OpenRISC implementation:
 * Copyright (C) 2025 Sahil Siddiq <sahilcdq@proton.me>
 */

// Dependencies supplied by the Linux cacheinfo, CPU info, SPR, and SPR
// definitions interfaces are referenced here but declared elsewhere.

#[inline]
unsafe fn ci_leaf_init(
    this_leaf: *mut cacheinfo,
    type_: cache_type,
    level: c_uint,
    cache: *mut cache_desc,
    cpu: c_int,
) {
    (*this_leaf).type_ = type_;
    (*this_leaf).level = level;
    (*this_leaf).coherency_line_size = (*cache).block_size;
    (*this_leaf).number_of_sets = (*cache).sets;
    (*this_leaf).ways_of_associativity = (*cache).ways;
    (*this_leaf).size = (*cache).size;
    cpumask_set_cpu(cpu, &mut (*this_leaf).shared_cpu_map);
}

pub unsafe fn init_cache_level(cpu: c_uint) -> c_int {
    let cpuinfo: *mut cpuinfo_or1k = &mut cpuinfo_or1k[smp_processor_id() as usize];
    let this_cpu_ci: *mut cpu_cacheinfo = get_cpu_cacheinfo(cpu);
    let mut leaves: c_int = 0;
    let mut levels: c_int = 0;
    let upr: c_ulong = mfspr(SPR_UPR);
    let mut iccfgr: c_ulong;
    let mut dccfgr: c_ulong;

    if upr & SPR_UPR_UP == 0 {
        printk!(KERN_INFO "-- no UPR register... unable to detect configuration\n");
        return -ENOENT;
    }

    if cpu_cache_is_present(SPR_UPR_DCP) {
        dccfgr = mfspr(SPR_DCCFGR);
        (*cpuinfo).dcache.ways = 1 << (dccfgr & SPR_DCCFGR_NCW);
        (*cpuinfo).dcache.sets = 1 << ((dccfgr & SPR_DCCFGR_NCS) >> 3);
        (*cpuinfo).dcache.block_size = 16 << ((dccfgr & SPR_DCCFGR_CBS) >> 7);
        (*cpuinfo).dcache.size = (*cpuinfo).dcache.sets * (*cpuinfo).dcache.ways
            * (*cpuinfo).dcache.block_size;
        leaves += 1;
        printk!(KERN_INFO "-- dcache: %d bytes total, %d bytes/line, %d set(s), %d way(s)\n",
            (*cpuinfo).dcache.size, (*cpuinfo).dcache.block_size,
            (*cpuinfo).dcache.sets, (*cpuinfo).dcache.ways);
    } else {
        printk!(KERN_INFO "-- dcache disabled\n");
    }

    if cpu_cache_is_present(SPR_UPR_ICP) {
        iccfgr = mfspr(SPR_ICCFGR);
        (*cpuinfo).icache.ways = 1 << (iccfgr & SPR_ICCFGR_NCW);
        (*cpuinfo).icache.sets = 1 << ((iccfgr & SPR_ICCFGR_NCS) >> 3);
        (*cpuinfo).icache.block_size = 16 << ((iccfgr & SPR_ICCFGR_CBS) >> 7);
        (*cpuinfo).icache.size = (*cpuinfo).icache.sets * (*cpuinfo).icache.ways
            * (*cpuinfo).icache.block_size;
        leaves += 1;
        printk!(KERN_INFO "-- icache: %d bytes total, %d bytes/line, %d set(s), %d way(s)\n",
            (*cpuinfo).icache.size, (*cpuinfo).icache.block_size,
            (*cpuinfo).icache.sets, (*cpuinfo).icache.ways);
    } else {
        printk!(KERN_INFO "-- icache disabled\n");
    }

    if leaves == 0 {
        return -ENOENT;
    }

    levels = 1;
    (*this_cpu_ci).num_leaves = leaves;
    (*this_cpu_ci).num_levels = levels;
    0
}

pub unsafe fn populate_cache_leaves(cpu: c_uint) -> c_int {
    let cpuinfo: *mut cpuinfo_or1k = &mut cpuinfo_or1k[smp_processor_id() as usize];
    let this_cpu_ci: *mut cpu_cacheinfo = get_cpu_cacheinfo(cpu);
    let mut this_leaf: *mut cacheinfo = (*this_cpu_ci).info_list;
    let level: c_int = 1;

    if cpu_cache_is_present(SPR_UPR_DCP) {
        ci_leaf_init(this_leaf, CACHE_TYPE_DATA, level, &mut (*cpuinfo).dcache, cpu as c_int);
        (*this_leaf).attributes = if (mfspr(SPR_DCCFGR) & SPR_DCCFGR_CWS) >> 8 != 0 {
            CACHE_WRITE_BACK
        } else {
            CACHE_WRITE_THROUGH
        };
        this_leaf = this_leaf.add(1);
    }

    if cpu_cache_is_present(SPR_UPR_ICP) {
        ci_leaf_init(this_leaf, CACHE_TYPE_INST, level, &mut (*cpuinfo).icache, cpu as c_int);
    }

    (*this_cpu_ci).cpu_map_populated = true;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
