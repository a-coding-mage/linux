// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2017 SiFive
 */

// Translated from the corresponding C implementation.  Kernel declarations
// supplied by the surrounding build provide the referenced types and symbols.

static mut rv_cache_ops: *mut riscv_cacheinfo_ops = core::ptr::null_mut();

pub unsafe fn riscv_set_cacheinfo_ops(ops: *mut riscv_cacheinfo_ops) {
    rv_cache_ops = ops;
}

pub unsafe fn cache_get_priv_group(
    this_leaf: *mut cacheinfo,
) -> *const attribute_group {
    if !rv_cache_ops.is_null() && !(*rv_cache_ops).get_priv_group.is_none() {
        return ((*rv_cache_ops).get_priv_group.unwrap())(this_leaf);
    }
    core::ptr::null()
}

unsafe fn get_cacheinfo(level: u32, type_: cache_type) -> *mut cacheinfo {
    /*
     * Using raw_smp_processor_id() elides a preemptability check, but this
     * is really indicative of a larger problem: the cacheinfo UABI assumes
     * that cores have a homonogenous view of the cache hierarchy.  That
     * happens to be the case for the current set of RISC-V systems, but
     * likely won't be true in general.  Since there's no way to provide
     * correct information for these systems via the current UABI we're
     * just eliding the check for now.
     */
    let this_cpu_ci: *mut cpu_cacheinfo =
        get_cpu_cacheinfo(raw_smp_processor_id());
    let mut this_leaf: *mut cacheinfo;
    let mut index: i32 = 0;

    while index < (*this_cpu_ci).num_leaves {
        this_leaf = (*this_cpu_ci).info_list.add(index as usize);
        if (*this_leaf).level == level && (*this_leaf).type_ == type_ {
            return this_leaf;
        }
        index += 1;
    }

    core::ptr::null_mut()
}

pub unsafe fn get_cache_size(level: u32, type_: cache_type) -> usize {
    let this_leaf = get_cacheinfo(level, type_);
    if !this_leaf.is_null() { (*this_leaf).size } else { 0 }
}

pub unsafe fn get_cache_geometry(level: u32, type_: cache_type) -> usize {
    let this_leaf = get_cacheinfo(level, type_);
    if !this_leaf.is_null() {
        ((*this_leaf).ways_of_associativity << 16) | (*this_leaf).coherency_line_size
    } else {
        0
    }
}

unsafe fn ci_leaf_init(this_leaf: *mut cacheinfo, type_: cache_type, level: u32) {
    (*this_leaf).level = level;
    (*this_leaf).type_ = type_;
}

pub unsafe fn init_cache_level(cpu: u32) -> i32 {
    init_of_cache_level(cpu)
}

pub unsafe fn populate_cache_leaves(cpu: u32) -> i32 {
    let this_cpu_ci = get_cpu_cacheinfo(cpu);
    let mut this_leaf = (*this_cpu_ci).info_list;
    let mut np: *mut device_node;
    let mut prev: *mut device_node;
    let mut levels: u32 = 1;
    let mut level: u32 = 1;

    if !acpi_disabled {
        let mut fw_levels: i32 = 0;
        let mut split_levels: i32 = 0;

        let ret = acpi_get_cache_info(cpu, &mut fw_levels, &mut split_levels);
        if ret != 0 { return ret; }

        BUG_ON(split_levels > fw_levels || split_levels + fw_levels > (*this_cpu_ci).num_leaves);

        while level <= (*this_cpu_ci).num_levels as u32 {
            if (level as i32) <= split_levels {
                ci_leaf_init(this_leaf, CACHE_TYPE_DATA, level); this_leaf = this_leaf.add(1);
                ci_leaf_init(this_leaf, CACHE_TYPE_INST, level); this_leaf = this_leaf.add(1);
            } else {
                ci_leaf_init(this_leaf, CACHE_TYPE_UNIFIED, level); this_leaf = this_leaf.add(1);
            }
            level += 1;
        }
        return 0;
    }

    np = of_cpu_device_node_get(cpu);
    if np.is_null() { return -ENOENT; }

    if of_property_present(np, b"cache-size\0") != 0 { ci_leaf_init(this_leaf, CACHE_TYPE_UNIFIED, level); this_leaf = this_leaf.add(1); }
    if of_property_present(np, b"i-cache-size\0") != 0 { ci_leaf_init(this_leaf, CACHE_TYPE_INST, level); this_leaf = this_leaf.add(1); }
    if of_property_present(np, b"d-cache-size\0") != 0 { ci_leaf_init(this_leaf, CACHE_TYPE_DATA, level); this_leaf = this_leaf.add(1); }

    prev = np;
    loop {
        np = of_find_next_cache_node(np);
        if np.is_null() { break; }
        of_node_put(prev); prev = np;
        if of_device_is_compatible(np, b"cache\0") == 0 { break; }
        if of_property_read_u32(np, b"cache-level\0", &mut level) != 0 { break; }
        if level <= levels { break; }
        if of_property_present(np, b"cache-size\0") != 0 { ci_leaf_init(this_leaf, CACHE_TYPE_UNIFIED, level); this_leaf = this_leaf.add(1); }
        if of_property_present(np, b"i-cache-size\0") != 0 { ci_leaf_init(this_leaf, CACHE_TYPE_INST, level); this_leaf = this_leaf.add(1); }
        if of_property_present(np, b"d-cache-size\0") != 0 { ci_leaf_init(this_leaf, CACHE_TYPE_DATA, level); this_leaf = this_leaf.add(1); }
        levels = level;
    }
    of_node_put(prev);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
