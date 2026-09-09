// SPDX-License-Identifier: GPL-2.0-only
/*
 * LoongArch cacheinfo support
 *
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// C dependencies: linux/cacheinfo.h, linux/topology.h, asm/bootinfo.h,
// and asm/cpu-info.h provide the following types, functions, globals, and
// constants in the surrounding kernel translation.

extern "C" {
    static mut current_cpu_data: CpuData;
    fn get_cpu_cacheinfo(cpu: ::core::ffi::c_uint) -> *mut CpuCacheinfo;
    fn cpumask_empty(mask: *const Cpumask) -> bool;
    fn cpumask_set_cpu(cpu: ::core::ffi::c_uint, mask: *mut Cpumask);
    fn for_each_online_cpu_next(cpu: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    fn cpu_to_node(cpu: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    fn cpus_are_siblings(cpu1: ::core::ffi::c_uint, cpu2: ::core::ffi::c_uint) -> bool;
}

#[repr(C)]
pub struct Cpumask {
    _private: [u8; 0],
}

#[repr(C)]
pub struct CacheDesc {
    pub type_: ::core::ffi::c_int,
    pub level: ::core::ffi::c_int,
    pub linesz: usize,
    pub sets: usize,
    pub ways: usize,
    pub flags: u8,
}

#[repr(C)]
pub struct Cacheinfo {
    pub type_: ::core::ffi::c_int,
    pub level: ::core::ffi::c_int,
    pub coherency_line_size: usize,
    pub number_of_sets: usize,
    pub ways_of_associativity: usize,
    pub size: usize,
    pub priv_: *mut u8,
    pub shared_cpu_map: Cpumask,
}

#[repr(C)]
pub struct CpuCacheinfo {
    pub num_levels: ::core::ffi::c_int,
    pub num_leaves: ::core::ffi::c_int,
    pub info_list: *mut Cacheinfo,
    pub cpu_map_populated: bool,
}

#[repr(C)]
pub struct CpuData {
    pub cache_leaves_present: ::core::ffi::c_int,
    pub cache_leaves: *mut CacheDesc,
}

const CACHE_PRIVATE: u8 = 1;

pub unsafe fn init_cache_level(cpu: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    let cache_present = current_cpu_data.cache_leaves_present;
    let this_cpu_ci = get_cpu_cacheinfo(cpu);

    (*this_cpu_ci).num_levels =
        (*current_cpu_data.cache_leaves.offset((cache_present - 1) as isize)).level;
    (*this_cpu_ci).num_leaves = cache_present;

    0
}

#[inline]
unsafe fn cache_leaves_are_shared(this_leaf: *mut Cacheinfo, sib_leaf: *mut Cacheinfo) -> bool {
    (!(*((*this_leaf).priv_) & CACHE_PRIVATE) != 0)
        && (!(*((*sib_leaf).priv_) & CACHE_PRIVATE) != 0)
}

unsafe fn cache_cpumap_setup(cpu: ::core::ffi::c_uint) {
    let this_cpu_ci = get_cpu_cacheinfo(cpu);

    for index in 0..(*this_cpu_ci).num_leaves {
        let this_leaf = (*this_cpu_ci).info_list.add(index as usize);
        // skip if shared_cpu_map is already populated
        if !cpumask_empty(&(*this_leaf).shared_cpu_map) {
            continue;
        }

        cpumask_set_cpu(cpu, &mut (*this_leaf).shared_cpu_map);
        let mut i = 0;
        while i < ::core::ffi::c_uint::MAX {
            let sib_cpu_ci = get_cpu_cacheinfo(i);

            if i == cpu || (*sib_cpu_ci).info_list.is_null()
                || (cpu_to_node(i) != cpu_to_node(cpu))
            {
                i = for_each_online_cpu_next(i);
                continue;
            }

            let sib_leaf = (*sib_cpu_ci).info_list.add(index as usize);
            // SMT cores share all caches
            if cpus_are_siblings(i, cpu) {
                cpumask_set_cpu(cpu, &mut (*sib_leaf).shared_cpu_map);
                cpumask_set_cpu(i, &mut (*this_leaf).shared_cpu_map);
            }
            // Node's cores share shared caches
            if cache_leaves_are_shared(this_leaf, sib_leaf) {
                cpumask_set_cpu(cpu, &mut (*sib_leaf).shared_cpu_map);
                cpumask_set_cpu(i, &mut (*this_leaf).shared_cpu_map);
            }
            i = for_each_online_cpu_next(i);
        }
    }
}

pub unsafe fn populate_cache_leaves(cpu: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    let cache_present = current_cpu_data.cache_leaves_present;
    let this_cpu_ci = get_cpu_cacheinfo(cpu);
    let mut this_leaf = (*this_cpu_ci).info_list;
    let cdesc = current_cpu_data.cache_leaves;

    for i in 0..cache_present {
        let cd = cdesc.offset(i as isize);

        (*this_leaf).type_ = (*cd).type_;
        (*this_leaf).level = (*cd).level;
        (*this_leaf).coherency_line_size = (*cd).linesz;
        (*this_leaf).number_of_sets = (*cd).sets;
        (*this_leaf).ways_of_associativity = (*cd).ways;
        (*this_leaf).size = (*cd).linesz * (*cd).sets * (*cd).ways;
        (*this_leaf).priv_ = &mut (*cd).flags;
        this_leaf = this_leaf.add(1);
    }

    cache_cpumap_setup(cpu);
    (*this_cpu_ci).cpu_map_populated = true;

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
