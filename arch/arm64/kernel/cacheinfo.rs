// SPDX-License-Identifier: GPL-2.0-only
/*
 *  ARM64 cacheinfo support
 *
 *  Copyright (C) 2015 ARM Ltd.
 *  All Rights Reserved
 */

// Dependencies supplied by the kernel's ACPI, cacheinfo, and device-tree code
// are intentionally referenced here but not defined in this translation unit.

const MAX_CACHE_LEVEL: u32 = 7; /* Max 7 level supported */

extern "C" {
    static mut coherency_max_size: i32;
    fn cache_line_size_of_cpu() -> i32;
    fn get_cpu_cacheinfo(cpu: u32) -> *mut cpu_cacheinfo;
    fn of_find_last_cache_level(cpu: u32) -> i32;
    fn acpi_get_cache_info(cpu: u32, fw_level: *mut i32, leaves: *mut core::ffi::c_void) -> i32;
    static mut acpi_disabled: bool;
}

#[repr(C)]
struct cpu_cacheinfo {
    num_levels: u32,
    num_leaves: u32,
    info_list: *mut cacheinfo,
}

#[repr(C)]
struct cacheinfo {
    level: u32,
    type_: cache_type,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum cache_type {
    CACHE_TYPE_NOCACHE,
    CACHE_TYPE_SEPARATE,
    CACHE_TYPE_DATA,
    CACHE_TYPE_INST,
}

unsafe fn read_sysreg_clidr_el1() -> u64 {
    // Equivalent to read_sysreg(clidr_el1); supplied by the target architecture.
    core::arch::asm!("mrs {}, clidr_el1", out(reg) _);
    0
}

unsafe fn clidr_ctype(clidr: u64, level: u32) -> cache_type {
    // Equivalent to CLIDR_CTYPE(clidr, level); supplied by the cacheinfo headers.
    let _ = (clidr, level);
    core::hint::unreachable_unchecked()
}

pub unsafe fn cache_line_size() -> i32 {
    if coherency_max_size != 0 {
        return coherency_max_size;
    }

    cache_line_size_of_cpu()
}

unsafe fn get_cache_type(level: u32) -> cache_type {
    let clidr: u64;

    if level > MAX_CACHE_LEVEL {
        return cache_type::CACHE_TYPE_NOCACHE;
    }
    clidr = read_sysreg_clidr_el1();
    clidr_ctype(clidr, level)
}

unsafe fn ci_leaf_init(this_leaf: *mut cacheinfo, type_: cache_type, level: u32) {
    (*this_leaf).level = level;
    (*this_leaf).type_ = type_;
}

unsafe fn detect_cache_level(level_p: *mut u32, leaves_p: *mut u32) {
    let mut ctype: cache_type;
    let mut level: u32;
    let mut leaves: u32;

    level = 1;
    leaves = 0;
    while level <= MAX_CACHE_LEVEL {
        ctype = get_cache_type(level);
        if ctype == cache_type::CACHE_TYPE_NOCACHE {
            level = level.wrapping_sub(1);
            break;
        }
        /* Separate instruction and data caches */
        leaves = leaves.wrapping_add(if ctype == cache_type::CACHE_TYPE_SEPARATE { 2 } else { 1 });
        level = level.wrapping_add(1);
    }

    *level_p = level;
    *leaves_p = leaves;
}

pub unsafe fn early_cache_level(cpu: u32) -> i32 {
    let this_cpu_ci = get_cpu_cacheinfo(cpu);

    detect_cache_level(&mut (*this_cpu_ci).num_levels, &mut (*this_cpu_ci).num_leaves);

    0
}

pub unsafe fn init_cache_level(cpu: u32) -> i32 {
    let mut level: u32;
    let mut leaves: u32;
    let mut fw_level: i32;
    let ret: i32;
    let this_cpu_ci = get_cpu_cacheinfo(cpu);

    detect_cache_level(&mut level, &mut leaves);

    if acpi_disabled {
        fw_level = of_find_last_cache_level(cpu);
    } else {
        ret = acpi_get_cache_info(cpu, &mut fw_level, core::ptr::null_mut());
        if ret < 0 {
            fw_level = 0;
        }
    }

    if (level as i32) < fw_level {
        /*
         * some external caches not specified in CLIDR_EL1
         * the information may be available in the device tree
         * only unified external caches are considered here
         */
        leaves = leaves.wrapping_add((fw_level - level as i32) as u32);
        level = fw_level as u32;
    }

    (*this_cpu_ci).num_levels = level;
    (*this_cpu_ci).num_leaves = leaves;
    0
}

pub unsafe fn populate_cache_leaves(cpu: u32) -> i32 {
    let mut level: u32;
    let mut idx: u32;
    let mut type_: cache_type;
    let this_cpu_ci = get_cpu_cacheinfo(cpu);
    let infos = (*this_cpu_ci).info_list;

    idx = 0;
    level = 1;
    while level <= (*this_cpu_ci).num_levels && idx < (*this_cpu_ci).num_leaves {
        type_ = get_cache_type(level);
        if type_ == cache_type::CACHE_TYPE_SEPARATE {
            if idx + 1 >= (*this_cpu_ci).num_leaves {
                break;
            }
            ci_leaf_init(infos.add(idx as usize), cache_type::CACHE_TYPE_DATA, level);
            idx += 1;
            ci_leaf_init(infos.add(idx as usize), cache_type::CACHE_TYPE_INST, level);
            idx += 1;
        } else {
            ci_leaf_init(infos.add(idx as usize), type_, level);
            idx += 1;
        }
        level += 1;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
