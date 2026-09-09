// SPDX-License-Identifier: GPL-2.0-only
/*
 * MIPS cacheinfo support
 */

// Dependency declarations and build-time definitions are supplied by the
// surrounding kernel translation unit.

/* Populates leaf and increments to next leaf */
macro_rules! populate_cache {
    ($cache:ident, $leaf:ident, $c_level:expr, $c_type:expr) => {{
        unsafe {
            (*$leaf).type_ = $c_type;
            (*$leaf).level = $c_level;
            (*$leaf).coherency_line_size = current_cpu_data.$cache.linesz;
            (*$leaf).number_of_sets = current_cpu_data.$cache.sets;
            (*$leaf).ways_of_associativity = current_cpu_data.$cache.ways;
            (*$leaf).size = current_cpu_data.$cache.linesz
                * current_cpu_data.$cache.sets
                * current_cpu_data.$cache.ways;
            $leaf = $leaf.add(1);
        }
    }};
}

pub unsafe fn init_cache_level(cpu: u32) -> i32 {
    let c: *mut cpuinfo_mips = &raw mut current_cpu_data;
    let this_cpu_ci: *mut cpu_cacheinfo = get_cpu_cacheinfo(cpu);
    let mut levels = 0;
    let mut leaves = 0;

    /*
     * If Dcache is not set, we assume the cache structures
     * are not properly initialized.
     */
    if (*c).dcache.waysize != 0 {
        levels += 1;
    } else {
        return -ENOENT;
    }

    leaves += if (*c).icache.waysize != 0 { 2 } else { 1 };

    if (*c).vcache.waysize != 0 {
        levels += 1;
        leaves += 1;
    }

    if (*c).scache.waysize != 0 {
        levels += 1;
        leaves += 1;
    }

    if (*c).tcache.waysize != 0 {
        levels += 1;
        leaves += 1;
    }

    (*this_cpu_ci).num_levels = levels;
    (*this_cpu_ci).num_leaves = leaves;
    0
}

unsafe fn fill_cpumask_siblings(cpu: i32, cpu_map: *mut cpumask_t) {
    let mut cpu1: i32;

    for_each_possible_cpu!(cpu1) {
        if cpus_are_siblings(cpu, cpu1) {
            cpumask_set_cpu(cpu1, cpu_map);
        }
    }
}

unsafe fn fill_cpumask_cluster(cpu: i32, cpu_map: *mut cpumask_t) {
    let mut cpu1: i32;
    let cluster = cpu_cluster(&cpu_data[cpu as usize]);

    for_each_possible_cpu!(cpu1) {
        if cpu_cluster(&cpu_data[cpu1 as usize]) == cluster {
            cpumask_set_cpu(cpu1, cpu_map);
        }
    }
}

pub unsafe fn populate_cache_leaves(cpu: u32) -> i32 {
    let c: *mut cpuinfo_mips = &raw mut current_cpu_data;
    let this_cpu_ci: *mut cpu_cacheinfo = get_cpu_cacheinfo(cpu);
    let mut this_leaf: *mut cacheinfo = (*this_cpu_ci).info_list;
    let mut level = 1;

    if (*c).icache.waysize != 0 {
        /* I/D caches are per core */
        fill_cpumask_siblings(cpu as i32, &mut (*this_leaf).shared_cpu_map);
        populate_cache!(dcache, this_leaf, level, CACHE_TYPE_DATA);
        fill_cpumask_siblings(cpu as i32, &mut (*this_leaf).shared_cpu_map);
        populate_cache!(icache, this_leaf, level, CACHE_TYPE_INST);
        level += 1;
    } else {
        populate_cache!(dcache, this_leaf, level, CACHE_TYPE_UNIFIED);
        level += 1;
    }

    if (*c).vcache.waysize != 0 {
        /* Vcache is per core as well */
        fill_cpumask_siblings(cpu as i32, &mut (*this_leaf).shared_cpu_map);
        populate_cache!(vcache, this_leaf, level, CACHE_TYPE_UNIFIED);
        level += 1;
    }

    if (*c).scache.waysize != 0 {
        /* Scache is per cluster */
        fill_cpumask_cluster(cpu as i32, &mut (*this_leaf).shared_cpu_map);
        populate_cache!(scache, this_leaf, level, CACHE_TYPE_UNIFIED);
        level += 1;
    }

    if (*c).tcache.waysize != 0 {
        populate_cache!(tcache, this_leaf, level, CACHE_TYPE_UNIFIED);
    }

    (*this_cpu_ci).cpu_map_populated = true;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
