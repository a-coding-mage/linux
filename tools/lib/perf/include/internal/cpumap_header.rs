/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from lib/perf/include/internal/cpumap.h. */
/* C dependencies: <linux/refcount.h>, <perf/cpumap.h>, <internal/rc_check.h>. */

/**
 * A sized, reference counted, sorted array of integers representing CPU
 * numbers. This is commonly used to capture which CPUs a PMU is associated
 * with. The indices into the cpumap are frequently used as they avoid having
 * gaps if CPU numbers were used. For events associated with a pid, rather than
 * a CPU, a single dummy map with an entry of -1 is used.
 */
#[repr(C)]
pub struct perf_cpu_map {
    pub refcnt: refcount_t,
    /** Length of the map array. */
    pub nr: ::std::os::raw::c_uint,
    /** The CPU values. */
    pub map: [perf_cpu; 0],
}

extern "C" {
    pub fn perf_cpu_map__alloc(nr_cpus: ::std::os::raw::c_uint) -> *mut perf_cpu_map;
    pub fn perf_cpu_map__idx(cpus: *const perf_cpu_map, cpu: perf_cpu) -> ::std::os::raw::c_int;
    pub fn perf_cpu_map__is_subset(a: *const perf_cpu_map, b: *const perf_cpu_map) -> bool;

    pub fn perf_cpu_map__set_nr(map: *mut perf_cpu_map, nr_cpus: ::std::os::raw::c_uint);
}

/* C macro dependency from <internal/rc_check.h>. */
extern "C" {
    fn RC_CHK_ACCESS(map: *mut perf_cpu_map) -> *mut perf_cpu_map;
}

#[inline]
pub unsafe fn perf_cpu_map__refcnt(map: *mut perf_cpu_map) -> *mut refcount_t {
    &mut (*RC_CHK_ACCESS(map)).refcnt
}
