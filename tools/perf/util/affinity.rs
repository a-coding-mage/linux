// SPDX-License-Identifier: GPL-2.0
/* Manage affinity to optimize IPIs inside the kernel perf API. */
/* C dependencies: sched.h, stdlib.h, linux/bitmap.h, linux/zalloc.h,
 * perf/cpumap.h, perf.h, cpumap.h, affinity.h
 */

use core::ffi::{c_int, c_uint, c_ulong};

#[repr(C)]
pub struct perf_cpu {
    pub cpu: c_int,
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct affinity {
    pub orig_cpus: *mut c_ulong,
    pub sched_cpus: *mut c_ulong,
    pub changed: bool,
}

#[repr(C)]
pub struct cpu_set_t {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn cpu__max_cpu() -> perf_cpu;
    fn bitmap_zalloc(nbits: c_int) -> *mut c_ulong;
    fn bitmap_zero(dst: *mut c_ulong, nbits: c_int);
    fn zfree(ptr: *mut *mut c_ulong);
    fn __set_bit(nr: c_int, addr: *mut c_ulong);
    fn __clear_bit(nr: c_int, addr: *mut c_ulong);
    fn sched_getaffinity(pid: c_int, cpusetsize: usize, mask: *mut cpu_set_t) -> c_int;
    fn sched_setaffinity(pid: c_int, cpusetsize: usize, mask: *const cpu_set_t) -> c_int;
}

/* perf_cpu_map__for_each_cpu_skip_any is a C iteration macro supplied by cpumap.h. */
unsafe extern "C" {
    fn perf_cpu_map__nr(cpumap: *const perf_cpu_map) -> c_int;
    fn perf_cpu_map__cpu(cpumap: *const perf_cpu_map, idx: c_int) -> perf_cpu;
}

unsafe fn get_cpu_set_size() -> c_int {
    let mut sz: c_int = cpu__max_cpu().cpu + 8 - 1;
    /*
     * sched_getaffinity doesn't like masks smaller than the kernel.
     * Hopefully that's big enough.
     */
    if sz < 4096 {
        sz = 4096;
    }
    sz / 8
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn affinity__setup(a: *mut affinity) -> c_int {
    let cpu_set_size: c_int = get_cpu_set_size();

    (*a).orig_cpus = bitmap_zalloc(cpu_set_size * 8);
    if (*a).orig_cpus.is_null() {
        return -1;
    }
    sched_getaffinity(
        0,
        cpu_set_size as usize,
        (*a).orig_cpus as *mut cpu_set_t,
    );
    (*a).sched_cpus = bitmap_zalloc(cpu_set_size * 8);
    if (*a).sched_cpus.is_null() {
        zfree(&mut (*a).orig_cpus);
        return -1;
    }
    bitmap_zero((*a).sched_cpus, cpu_set_size);
    (*a).changed = false;
    0
}

/*
 * perf_event_open does an IPI internally to the target CPU.
 * It is more efficient to change perf's affinity to the target
 * CPU and then set up all events on that CPU, so we amortize
 * CPU communication.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn affinity__set(a: *mut affinity, cpu: c_int) {
    let cpu_set_size: c_int = get_cpu_set_size();

    /*
     * Return:
     * - if cpu is -1
     * - restrict out of bound access to sched_cpus
     */
    if cpu == -1 || cpu >= cpu_set_size * 8 {
        return;
    }

    (*a).changed = true;
    __set_bit(cpu, (*a).sched_cpus);
    /*
     * We ignore errors because affinity is just an optimization.
     * This could happen for example with isolated CPUs or cpusets.
     * In this case the IPIs inside the kernel's perf API still work.
     */
    sched_setaffinity(
        0,
        cpu_set_size as usize,
        (*a).sched_cpus as *mut cpu_set_t,
    );
    __clear_bit(cpu, (*a).sched_cpus);
}

unsafe fn __affinity__cleanup(a: *mut affinity) {
    let cpu_set_size: c_int = get_cpu_set_size();

    if (*a).changed {
        sched_setaffinity(
            0,
            cpu_set_size as usize,
            (*a).orig_cpus as *mut cpu_set_t,
        );
    }
    zfree(&mut (*a).sched_cpus);
    zfree(&mut (*a).orig_cpus);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn affinity__cleanup(a: *mut affinity) {
    if !a.is_null() {
        __affinity__cleanup(a);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cpu_map__set_affinity(cpumap: *const perf_cpu_map) {
    let cpu_set_size: c_int = get_cpu_set_size();
    let mut cpuset: *mut c_ulong = bitmap_zalloc(cpu_set_size * 8);
    let mut cpu: perf_cpu;
    let mut idx: c_uint;

    if cpuset.is_null() {
        return;
    }

    idx = 0;
    while (idx as c_int) < perf_cpu_map__nr(cpumap) {
        cpu = perf_cpu_map__cpu(cpumap, idx as c_int);
        if cpu.cpu != -1 {
            __set_bit(cpu.cpu, cpuset);
        }
        idx += 1;
    }

    sched_setaffinity(0, cpu_set_size as usize, cpuset as *mut cpu_set_t);
    zfree(&mut cpuset);
}
