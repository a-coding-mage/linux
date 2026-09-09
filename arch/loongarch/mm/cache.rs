// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 *
 * Derived from MIPS:
 * Copyright (C) 1994 - 2003, 06, 07 by Ralf Baechle (ralf@linux-mips.org)
 * Copyright (C) 2007 MIPS Technologies, Inc.
 */

// Linux and LoongArch headers supply the declarations used below.

extern "C" {
    static mut current_cpu_data: CpuData;
    static mut loongson_sysconf: LoongsonSysconf;
    static mut except_vec_cex: u8;

    fn set_merr_handler(addr: u64, handler: *mut u8, size: u64);
    fn flush_cache_line(leaf: c_uint, addr: u64);
    fn read_cpucfg(reg: c_uint) -> c_uint;
    fn bug_on(condition: bool);
    fn cache_private(cdesc: *const CacheDesc) -> bool;
    fn cache_inclusive(cdesc: *const CacheDesc) -> bool;
}

type c_uint = u32;
type pgprot_t = usize;

#[repr(C)]
struct CacheDesc {
    type_: c_uint,
    level: c_uint,
    flags: c_uint,
    ways: c_uint,
    sets: c_uint,
    linesz: c_uint,
}

#[repr(C)]
struct CpuData {
    cache_leaves: *mut CacheDesc,
    cache_leaves_present: c_uint,
    options: c_uint,
}

#[repr(C)]
struct LoongsonSysconf {
    nr_nodes: c_int,
}

type c_int = i32;

const CSR_DMW0_BASE: u64 = 0; // supplied by asm/loongarch.h
const NODE_ADDRSPACE_SHIFT: u32 = 0; // supplied by asm/numa.h
const CACHE_PRIVATE: c_uint = 1 << 0; // supplied by linux/cacheinfo.h
const CACHE_INCLUSIVE: c_uint = 1 << 1;
const CACHE_PRESENT: c_uint = 1 << 2;
const CACHE_TYPE_UNIFIED: c_uint = 1;
const CACHE_TYPE_INST: c_uint = 2;
const CACHE_TYPE_DATA: c_uint = 3;
const CACHE_LEVEL_MAX: c_uint = 4; // supplied by linux/cacheinfo.h
const CACHE_LEAVES_MAX: c_uint = 8;
const LOONGARCH_CPU_PREFETCH: c_uint = 1 << 0;
const LOONGARCH_CPUCFG16: c_uint = 16;
const LOONGARCH_CPUCFG17: c_uint = 17;
const CPUCFG_CACHE_WAYS_M: c_uint = 0;
const CPUCFG_CACHE_WAYS: u32 = 0;
const CPUCFG_CACHE_SETS_M: c_uint = 0;
const CPUCFG_CACHE_SETS: u32 = 0;
const CPUCFG_CACHE_LSIZE_M: c_uint = 0;
const CPUCFG_CACHE_LSIZE: u32 = 0;

pub unsafe fn cache_error_setup() {
    set_merr_handler(0x0, &raw mut except_vec_cex, 0x80);
}

unsafe fn flush_cache_leaf(leaf: c_uint) {
    let mut addr: u64 = CSR_DMW0_BASE;
    let cdesc = (*current_cpu_data.cache_leaves.add(leaf as usize)) as *const CacheDesc;
    let mut nr_nodes: c_int = if cache_private(cdesc) { 1 } else { loongson_sysconf.nr_nodes };

    loop {
        let mut i: c_uint = 0;
        while i < (*cdesc).sets {
            let mut j: c_uint = 0;
            while j < (*cdesc).ways {
                flush_cache_line(leaf, addr);
                addr = addr.wrapping_add(1);
                j += 1;
            }
            addr = addr.wrapping_sub((*cdesc).ways as u64);
            addr = addr.wrapping_add((*cdesc).linesz as u64);
            i += 1;
        }
        addr = addr.wrapping_add(1u64 << NODE_ADDRSPACE_SHIFT);
        nr_nodes -= 1;
        if nr_nodes <= 0 { break; }
    }
}

#[no_mangle]
pub unsafe extern "C" fn __flush_cache_all() {
    let cache_present = current_cpu_data.cache_leaves_present;
    let mut leaf = cache_present - 1;
    if cache_inclusive(current_cpu_data.cache_leaves.add(leaf as usize)) {
        flush_cache_leaf(leaf);
        return;
    }
    leaf = 0;
    while leaf < cache_present {
        flush_cache_leaf(leaf);
        leaf += 1;
    }
}

const L1IUPRE: c_uint = 1 << 0;
const L1IUUNIFY: c_uint = 1 << 1;
const L1DPRE: c_uint = 1 << 2;
const LXIUPRE: c_uint = 1 << 0;
const LXIUUNIFY: c_uint = 1 << 1;
const LXIUPRIV: c_uint = 1 << 2;
const LXIUINCL: c_uint = 1 << 3;
const LXDPRE: c_uint = 1 << 4;

unsafe fn populate_cache_properties(cfg0: c_uint, cdesc: &mut *mut CacheDesc,
                                    level: c_uint, leaf: &mut c_uint) {
    let cfg1 = read_cpucfg(LOONGARCH_CPUCFG17 + *leaf);
    if level == 1 { (*(*cdesc)).flags |= CACHE_PRIVATE; }
    else {
        if cfg0 & (1 << 2) != 0 { (*(*cdesc)).flags |= CACHE_PRIVATE; }
        if cfg0 & (1 << 3) != 0 { (*(*cdesc)).flags |= CACHE_INCLUSIVE; }
    }
    (*(*cdesc)).level = level;
    (*(*cdesc)).flags |= CACHE_PRESENT;
    (*(*cdesc)).ways = ((cfg1 & CPUCFG_CACHE_WAYS_M) >> CPUCFG_CACHE_WAYS) + 1;
    (*(*cdesc)).sets = 1 << ((cfg1 & CPUCFG_CACHE_SETS_M) >> CPUCFG_CACHE_SETS);
    (*(*cdesc)).linesz = 1 << ((cfg1 & CPUCFG_CACHE_LSIZE_M) >> CPUCFG_CACHE_LSIZE);
    *cdesc = (*cdesc).add(1);
    *leaf += 1;
}

pub unsafe fn cpu_cache_init() {
    let mut leaf = 0;
    let mut level = 1;
    let mut config = read_cpucfg(LOONGARCH_CPUCFG16);
    let mut cdesc = current_cpu_data.cache_leaves;
    if config & L1IUPRE != 0 {
        (*cdesc).type_ = if config & L1IUUNIFY != 0 { CACHE_TYPE_UNIFIED } else { CACHE_TYPE_INST };
        populate_cache_properties(config, &mut cdesc, level, &mut leaf);
    }
    if config & L1DPRE != 0 {
        (*cdesc).type_ = CACHE_TYPE_DATA;
        populate_cache_properties(config, &mut cdesc, level, &mut leaf);
    }
    config >>= 3;
    while level <= CACHE_LEVEL_MAX {
        if config == 0 { break; }
        if config & LXIUPRE != 0 {
            (*cdesc).type_ = if config & LXIUUNIFY != 0 { CACHE_TYPE_UNIFIED } else { CACHE_TYPE_INST };
            populate_cache_properties(config, &mut cdesc, level, &mut leaf);
        }
        if config & LXDPRE != 0 {
            (*cdesc).type_ = CACHE_TYPE_DATA;
            populate_cache_properties(config, &mut cdesc, level, &mut leaf);
        }
        config >>= 7;
        level += 1;
    }
    bug_on(leaf > CACHE_LEAVES_MAX);
    current_cpu_data.cache_leaves_present = leaf;
    current_cpu_data.options |= LOONGARCH_CPU_PREFETCH;
}

// protection_map is supplied by the architecture's page-protection definitions.
extern "C" {
    static protection_map: [pgprot_t; 16];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
