// SPDX-License-Identifier: GPL-2.0
/*
 * Extract CPU cache information and expose them via sysfs.
 *
 *    Copyright IBM Corp. 2012
 */

// Linux kernel dependencies supplied by other translation units.

const CACHE_SCOPE_NOTEXISTS: u32 = 0;
const CACHE_SCOPE_PRIVATE: u32 = 1;
const CACHE_SCOPE_SHARED: u32 = 2;
const CACHE_SCOPE_RESERVED: u32 = 3;

const CTYPE_SEPARATE: usize = 0;
const CTYPE_DATA: usize = 1;
const CTYPE_INSTRUCTION: usize = 2;
const CTYPE_UNIFIED: usize = 3;

const EXTRACT_TOPOLOGY: i32 = 0;
const EXTRACT_LINE_SIZE: i32 = 1;
const EXTRACT_SIZE: i32 = 2;
const EXTRACT_ASSOCIATIVITY: i32 = 3;

const CACHE_TI_UNIFIED: i32 = 0;
const CACHE_TI_DATA: i32 = 0;
const CACHE_TI_INSTRUCTION: i32 = 1;

#[repr(C)]
pub struct cache_info {
    pub bits: u8,
}

impl cache_info {
    #[inline]
    fn scope(&self) -> u8 { (self.bits >> 4) & 3 }
    #[inline]
    fn cache_type(&self) -> u8 { (self.bits >> 6) & 3 }
}

const CACHE_MAX_LEVEL: usize = 8;

#[repr(C)]
pub union cache_topology {
    pub ci: [cache_info; CACHE_MAX_LEVEL],
    pub raw: usize,
}

static CACHE_TYPE_STRING: [&[u8]; 5] = [b"", b"Instruction", b"Data", b"", b"Unified"];

// `enum cache_type` and the cache-info structures/functions are provided by
// the corresponding Linux cacheinfo translation unit.
type cache_type = i32;
const CACHE_TYPE_SEPARATE: cache_type = 0;
const CACHE_TYPE_INST: cache_type = 1;
const CACHE_TYPE_UNIFIED: cache_type = 2;
const CACHE_TYPE_NOCACHE: cache_type = 3;

static CACHE_TYPE_MAP: [cache_type; 4] = [
    CACHE_TYPE_SEPARATE,
    CACHE_TYPE_SEPARATE,
    CACHE_TYPE_INST,
    CACHE_TYPE_UNIFIED,
];

pub unsafe fn show_cacheinfo(m: *mut seq_file) {
    let this_cpu_ci = get_cpu_cacheinfo(cpumask_any(cpu_online_mask));
    let mut idx = 0;
    while idx < (*this_cpu_ci).num_leaves {
        let cache = (*this_cpu_ci).info_list.add(idx as usize);
        seq_printf(m, b"cache%-11d: \0".as_ptr(), idx);
        seq_printf(m, b"level=%d \0".as_ptr(), (*cache).level);
        seq_printf(m, b"type=%s \0".as_ptr(), CACHE_TYPE_STRING[(*cache).ty as usize].as_ptr());
        seq_printf(m, b"scope=%s \0".as_ptr(),
                   if (*cache).disable_sysfs { b"Shared\0".as_ptr() } else { b"Private\0".as_ptr() });
        seq_printf(m, b"size=%dK \0".as_ptr(), (*cache).size >> 10);
        seq_printf(m, b"line_size=%u \0".as_ptr(), (*cache).coherency_line_size);
        seq_printf(m, b"associativity=%d\0".as_ptr(), (*cache).ways_of_associativity);
        seq_puts(m, b"\n\0".as_ptr());
        idx += 1;
    }
}

#[inline]
unsafe fn get_cache_type(ci: *mut cache_info, level: i32) -> cache_type {
    if level >= CACHE_MAX_LEVEL as i32 {
        return CACHE_TYPE_NOCACHE;
    }
    let ci = &*ci.add(level as usize);
    let scope = ci.scope() as u32;
    if scope != CACHE_SCOPE_SHARED && scope != CACHE_SCOPE_PRIVATE {
        return CACHE_TYPE_NOCACHE;
    }
    CACHE_TYPE_MAP[ci.cache_type() as usize]
}

#[inline]
unsafe fn ecag(ai: i32, li: i32, ti: i32) -> usize {
    __ecag(ECAG_CACHE_ATTRIBUTE, ((ai << 4) | (li << 1) | ti) as u32)
}

unsafe fn ci_leaf_init(this_leaf: *mut cacheinfo, private: i32,
                       ty: cache_type, level: u32, cpu: i32) {
    let ti: i32 = if ty == CACHE_TYPE_INST { CACHE_TI_INSTRUCTION } else { CACHE_TI_UNIFIED };
    (*this_leaf).level = level + 1;
    (*this_leaf).ty = ty;
    (*this_leaf).coherency_line_size = ecag(EXTRACT_LINE_SIZE, level as i32, ti) as u32;
    (*this_leaf).ways_of_associativity = ecag(EXTRACT_ASSOCIATIVITY, level as i32, ti) as i32;
    (*this_leaf).size = ecag(EXTRACT_SIZE, level as i32, ti) as i32;
    let mut num_sets = (*this_leaf).size / (*this_leaf).coherency_line_size as i32;
    num_sets /= (*this_leaf).ways_of_associativity;
    (*this_leaf).number_of_sets = num_sets;
    cpumask_set_cpu(cpu, &mut (*this_leaf).shared_cpu_map);
    if private == 0 { (*this_leaf).disable_sysfs = true; }
}

pub unsafe fn init_cache_level(cpu: u32) -> i32 {
    let this_cpu_ci = get_cpu_cacheinfo(cpu);
    if this_cpu_ci.is_null() { return -22; }
    let mut level: u32 = 0;
    let mut leaves: u32 = 0;
    let mut ct = cache_topology { raw: ecag(EXTRACT_TOPOLOGY, 0, 0) };
    let ci = ct.ci.as_mut_ptr();
    loop {
        let ctype = get_cache_type(ci, level as i32);
        if ctype == CACHE_TYPE_NOCACHE { break; }
        leaves += if ctype == CACHE_TYPE_SEPARATE { 2 } else { 1 };
        level += 1;
        if level >= CACHE_MAX_LEVEL as u32 { break; }
    }
    (*this_cpu_ci).num_levels = level;
    (*this_cpu_ci).num_leaves = leaves;
    0
}

pub unsafe fn populate_cache_leaves(cpu: u32) -> i32 {
    let this_cpu_ci = get_cpu_cacheinfo(cpu);
    let mut this_leaf = (*this_cpu_ci).info_list;
    let mut ct = cache_topology { raw: ecag(EXTRACT_TOPOLOGY, 0, 0) };
    let ci = ct.ci.as_mut_ptr();
    let mut idx = 0;
    let mut level = 0;
    while level < (*this_cpu_ci).num_levels && idx < (*this_cpu_ci).num_leaves {
        if this_leaf.is_null() { return -22; }
        let pvt = if (*ci.add(level as usize)).scope() as u32 == CACHE_SCOPE_PRIVATE { 1 } else { 0 };
        let ctype = get_cache_type(ci, level as i32);
        if ctype == CACHE_TYPE_SEPARATE {
            ci_leaf_init(this_leaf, pvt, CACHE_TYPE_DATA, level, cpu as i32);
            this_leaf = this_leaf.add(1);
            ci_leaf_init(this_leaf, pvt, CACHE_TYPE_INST, level, cpu as i32);
            this_leaf = this_leaf.add(1);
        } else {
            ci_leaf_init(this_leaf, pvt, ctype, level, cpu as i32);
            this_leaf = this_leaf.add(1);
        }
        idx += 1;
        level += 1;
    }
    (*this_cpu_ci).cpu_map_populated = true;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
