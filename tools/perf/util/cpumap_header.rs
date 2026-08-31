/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Translated from perf/util/cpumap.h.
 *
 * C header dependencies removed from executable Rust:
 * - <stdbool.h>
 * - <stdio.h>
 * - <perf/cpumap.h>
 *
 * The perf_cpu, perf_cpu_map, and FILE types, along with perf_cpu_map__nr and
 * perf_cpu_map__cpu, are supplied by those external dependencies.
 */

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    pub type FILE;
    pub type perf_cpu_map;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_cpu {
    pub cpu: c_int,
}

/** Identify where counts are aggregated, -1 implies not to aggregate. */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aggr_cpu_id {
    /** A value in the range 0 to number of threads. */
    pub thread_idx: c_int,
    /** The numa node X as read from /sys/devices/system/node/nodeX. */
    pub node: c_int,
    /**
     * The socket number as read from
     * /sys/devices/system/cpu/cpuX/topology/physical_package_id.
     */
    pub socket: c_int,
    /** The die id as read from /sys/devices/system/cpu/cpuX/topology/die_id. */
    pub die: c_int,
    /** The cluster id as read from /sys/devices/system/cpu/cpuX/topology/cluster_id */
    pub cluster: c_int,
    /** The cache level as read from /sys/devices/system/cpu/cpuX/cache/indexY/level */
    pub cache_lvl: c_int,
    /**
     * The cache instance ID, which is the first CPU in the
     * /sys/devices/system/cpu/cpuX/cache/indexY/shared_cpu_list
     */
    pub cache: c_int,
    /** The core id as read from /sys/devices/system/cpu/cpuX/topology/core_id. */
    pub core: c_int,
    /** CPU aggregation, note there is one CPU for each SMT thread. */
    pub cpu: perf_cpu,
}

/** A collection of aggr_cpu_id values, the "built" version is sorted and uniqued. */
#[repr(C)]
pub struct cpu_aggr_map {
    /** Number of valid entries. */
    pub nr: c_int,
    /** The entries. C flexible array member: struct aggr_cpu_id map[]; */
    pub map: [aggr_cpu_id; 0],
}

/*
 * C macro:
 * #define cpu_aggr_map__for_each_idx(idx, aggr_map) \
 *      for ((idx) = 0; (idx) < aggr_map->nr; (idx)++)
 *
 * Rust callers should spell out the equivalent range iteration over 0..nr.
 */

#[repr(C)]
pub struct perf_record_cpu_map_data {
    _private: [u8; 0],
}

pub type aggr_cpu_id_get_t =
    Option<unsafe extern "C" fn(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id>;

extern "C" {
    pub fn perf_record_cpu_map_data__test_bit(
        i: c_int,
        data: *const perf_record_cpu_map_data,
    ) -> bool;

    pub fn perf_cpu_map__empty_new(nr: c_int) -> *mut perf_cpu_map;

    pub fn cpu_map__new_data(data: *const perf_record_cpu_map_data) -> *mut perf_cpu_map;
    pub fn cpu_map__snprint(map: *mut perf_cpu_map, buf: *mut c_char, size: usize) -> usize;
    pub fn cpu_map__snprint_mask(map: *mut perf_cpu_map, buf: *mut c_char, size: usize) -> usize;
    pub fn cpu_map__fprintf(map: *mut perf_cpu_map, fp: *mut FILE) -> usize;
    pub fn cpu_map__online() -> *mut perf_cpu_map; /* thread unsafe */

    pub fn cpu__setup_cpunode_map() -> c_int;

    pub fn cpu__max_node() -> c_int;
    pub fn cpu__max_cpu() -> perf_cpu;
    pub fn cpu__max_present_cpu() -> perf_cpu;

    pub fn perf_cpu_map__nr(cpus: *const perf_cpu_map) -> c_int;
    pub fn perf_cpu_map__cpu(cpus: *const perf_cpu_map, idx: c_int) -> perf_cpu;
}

/**
 * cpu_map__is_dummy - Events associated with a pid, rather than a CPU, use a single dummy map with an entry of -1.
 */
#[inline]
pub unsafe fn cpu_map__is_dummy(cpus: *const perf_cpu_map) -> bool {
    unsafe { perf_cpu_map__nr(cpus) == 1 && perf_cpu_map__cpu(cpus, 0).cpu == -1 }
}

extern "C" {
    /**
     * cpu__get_node - Returns the numa node X as read from
     * /sys/devices/system/node/nodeX for the given CPU.
     */
    pub fn cpu__get_node(cpu: perf_cpu) -> c_int;
    /**
     * cpu__get_socket_id - Returns the socket number as read from
     * /sys/devices/system/cpu/cpuX/topology/physical_package_id for the given CPU.
     */
    pub fn cpu__get_socket_id(cpu: perf_cpu) -> c_int;
    /**
     * cpu__get_die_id - Returns the die id as read from
     * /sys/devices/system/cpu/cpuX/topology/die_id for the given CPU.
     */
    pub fn cpu__get_die_id(cpu: perf_cpu) -> c_int;
    /**
     * cpu__get_cluster_id - Returns the cluster id as read from
     * /sys/devices/system/cpu/cpuX/topology/cluster_id for the given CPU
     */
    pub fn cpu__get_cluster_id(cpu: perf_cpu) -> c_int;
    /**
     * cpu__get_core_id - Returns the core id as read from
     * /sys/devices/system/cpu/cpuX/topology/core_id for the given CPU.
     */
    pub fn cpu__get_core_id(cpu: perf_cpu) -> c_int;

    /**
     * cpu_aggr_map__empty_new - Create a cpu_aggr_map of size nr with every entry
     * being empty.
     */
    pub fn cpu_aggr_map__empty_new(nr: c_int) -> *mut cpu_aggr_map;

    /**
     * cpu_aggr_map__new - Create a cpu_aggr_map with an aggr_cpu_id for each cpu in
     * cpus. The aggr_cpu_id is created with 'get_id' that may have a data value
     * passed to it. The cpu_aggr_map is sorted with duplicate values removed.
     */
    pub fn cpu_aggr_map__new(
        cpus: *const perf_cpu_map,
        get_id: aggr_cpu_id_get_t,
        data: *mut c_void,
        needs_sort: bool,
    ) -> *mut cpu_aggr_map;

    pub fn aggr_cpu_id__equal(a: *const aggr_cpu_id, b: *const aggr_cpu_id) -> bool;
    pub fn aggr_cpu_id__is_empty(a: *const aggr_cpu_id) -> bool;
    pub fn aggr_cpu_id__empty() -> aggr_cpu_id;

    /**
     * aggr_cpu_id__socket - Create an aggr_cpu_id with the socket populated with
     * the socket for cpu. The function signature is compatible with
     * aggr_cpu_id_get_t.
     */
    pub fn aggr_cpu_id__socket(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id;
    /**
     * aggr_cpu_id__die - Create an aggr_cpu_id with the die and socket populated
     * with the die and socket for cpu. The function signature is compatible with
     * aggr_cpu_id_get_t.
     */
    pub fn aggr_cpu_id__die(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id;
    /**
     * aggr_cpu_id__cluster - Create an aggr_cpu_id with cluster, die and socket
     * populated with the cluster, die and socket for cpu. The function signature
     * is compatible with aggr_cpu_id_get_t.
     */
    pub fn aggr_cpu_id__cluster(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id;
    /**
     * aggr_cpu_id__core - Create an aggr_cpu_id with the core, cluster, die and
     * socket populated with the core, die and socket for cpu. The function
     * signature is compatible with aggr_cpu_id_get_t.
     */
    pub fn aggr_cpu_id__core(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id;
    /**
     * aggr_cpu_id__core - Create an aggr_cpu_id with the cpu, core, die and socket
     * populated with the cpu, core, die and socket for cpu. The function signature
     * is compatible with aggr_cpu_id_get_t.
     */
    pub fn aggr_cpu_id__cpu(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id;
    /**
     * aggr_cpu_id__node - Create an aggr_cpu_id with the numa node populated for
     * cpu. The function signature is compatible with aggr_cpu_id_get_t.
     */
    pub fn aggr_cpu_id__node(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id;
    /**
     * aggr_cpu_id__global - Create an aggr_cpu_id for global aggregation.
     * The function signature is compatible with aggr_cpu_id_get_t.
     */
    pub fn aggr_cpu_id__global(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id;
}
