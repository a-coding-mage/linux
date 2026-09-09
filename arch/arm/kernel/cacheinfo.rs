// SPDX-License-Identifier: GPL-2.0-only
/*
 *  ARM cacheinfo support
 *
 *  Copyright (C) 2023 Linaro Ltd.
 *  Copyright (C) 2015 ARM Ltd.
 *  All Rights Reserved
 */

// Dependencies supplied by the surrounding kernel translation.

const MAX_CACHE_LEVEL: u32 = 7;

const CTR_FORMAT_MASK: u32 = 0xe000_0000;
const CTR_FORMAT_ARMV6: u32 = 0;
const CTR_FORMAT_ARMV7: u32 = 4;
const CTR_CWG_MASK: u32 = 0x0f00_0000;
const CTR_DSIZE_LEN_MASK: u32 = 0x0000_3000;
const CTR_ISIZE_LEN_MASK: u32 = 0x0000_0003;

#[inline]
const fn clidr_ctype_shift(level: u32) -> u32 {
    3 * (level - 1)
}

#[inline]
const fn clidr_ctype_mask(level: u32) -> u32 {
    7 << clidr_ctype_shift(level)
}

#[inline]
fn clidr_ctype(clidr: u32, level: u32) -> u32 {
    (clidr & clidr_ctype_mask(level)) >> clidr_ctype_shift(level)
}

#[inline]
unsafe fn cache_line_size_cp15() -> i32 {
    let ctr: u32 = read_cpuid_cachetype();
    let format = (ctr & CTR_FORMAT_MASK) >> 29;

    if format == CTR_FORMAT_ARMV7 {
        let cwg = (ctr & CTR_CWG_MASK) >> 24;
        if cwg != 0 {
            (4u32 << cwg) as i32
        } else {
            ARCH_DMA_MINALIGN
        }
    } else if format != CTR_FORMAT_ARMV6 {
        // WARN_ON_ONCE(format != CTR_FORMAT_ARMV6)
        ARCH_DMA_MINALIGN
    } else {
        let isize_len = ctr & CTR_ISIZE_LEN_MASK;
        let dsize_len = (ctr & CTR_DSIZE_LEN_MASK) >> 12;
        8i32 << core::cmp::max(isize_len, dsize_len)
    }
}

pub unsafe fn cache_line_size() -> i32 {
    if coherency_max_size != 0 {
        return coherency_max_size;
    }

    /* CP15 is optional / implementation defined before ARMv6 */
    if cpu_architecture() < CPU_ARCH_ARMV6 {
        return ARCH_DMA_MINALIGN;
    }

    cache_line_size_cp15()
}

#[inline]
unsafe fn get_cache_type(level: i32) -> cache_type {
    if level > MAX_CACHE_LEVEL as i32 {
        return CACHE_TYPE_NOCACHE;
    }

    let clidr = read_clidr();
    clidr_ctype(clidr, level as u32) as cache_type
}

unsafe fn ci_leaf_init(this_leaf: *mut cacheinfo, type_: cache_type, level: u32) {
    (*this_leaf).level = level;
    (*this_leaf).type_ = type_;
}

unsafe fn detect_cache_level(level_p: *mut u32, leaves_p: *mut u32) -> i32 {
    let mut ctype: u32;
    let mut level: u32;
    let mut leaves: u32;
    let ctr: u32;
    let format: u32;

    /* CLIDR is not present before ARMv7/v7m */
    if cpu_architecture() < CPU_ARCH_ARMV7 {
        return -EOPNOTSUPP;
    }

    /* Don't try reading CLIDR if CTR declares old format */
    ctr = read_cpuid_cachetype();
    format = (ctr & CTR_FORMAT_MASK) >> 29;
    if format != CTR_FORMAT_ARMV7 {
        return -EOPNOTSUPP;
    }

    level = 1;
    leaves = 0;
    while level <= MAX_CACHE_LEVEL {
        ctype = get_cache_type(level as i32) as u32;
        if ctype == CACHE_TYPE_NOCACHE as u32 {
            level -= 1;
            break;
        }
        /* Separate instruction and data caches */
        leaves += if ctype == CACHE_TYPE_SEPARATE as u32 { 2 } else { 1 };
        level += 1;
    }

    *level_p = level;
    *leaves_p = leaves;
    0
}

pub unsafe fn early_cache_level(cpu: u32) -> i32 {
    let this_cpu_ci = get_cpu_cacheinfo(cpu);
    detect_cache_level(&mut (*this_cpu_ci).num_levels, &mut (*this_cpu_ci).num_leaves)
}

pub unsafe fn init_cache_level(cpu: u32) -> i32 {
    let mut level: u32 = 0;
    let mut leaves: u32 = 0;
    let this_cpu_ci = get_cpu_cacheinfo(cpu);
    let fw_level: i32;
    let ret = detect_cache_level(&mut level, &mut leaves);
    if ret != 0 {
        return ret;
    }

    fw_level = of_find_last_cache_level(cpu);
    if (level as i32) < fw_level {
        /*
         * some external caches not specified in CLIDR_EL1
         * the information may be available in the device tree
         * only unified external caches are considered here
         */
        leaves += (fw_level as u32) - level;
        level = fw_level as u32;
    }

    (*this_cpu_ci).num_levels = level;
    (*this_cpu_ci).num_leaves = leaves;
    0
}

pub unsafe fn populate_cache_leaves(cpu: u32) -> i32 {
    let mut level: u32;
    let mut idx: u32 = 0;
    let mut type_: cache_type;
    let this_cpu_ci = get_cpu_cacheinfo(cpu);
    let mut this_leaf = (*this_cpu_ci).info_list;
    let arch = cpu_architecture();

    /* CLIDR is not present before ARMv7/v7m */
    if arch < CPU_ARCH_ARMV7 {
        return -EOPNOTSUPP;
    }

    level = 1;
    while level <= (*this_cpu_ci).num_levels && idx < (*this_cpu_ci).num_leaves {
        type_ = get_cache_type(level as i32);
        if type_ == CACHE_TYPE_SEPARATE {
            ci_leaf_init(this_leaf, CACHE_TYPE_DATA, level);
            this_leaf = this_leaf.add(1);
            ci_leaf_init(this_leaf, CACHE_TYPE_INST, level);
            this_leaf = this_leaf.add(1);
        } else {
            ci_leaf_init(this_leaf, type_, level);
            this_leaf = this_leaf.add(1);
        }
        idx += 1;
        level += 1;
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
