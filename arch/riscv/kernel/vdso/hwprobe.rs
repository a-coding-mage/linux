// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2023 Rivos, Inc
 */

// Dependencies supplied by the surrounding kernel/vDSO environment.

use core::ffi::{c_int, c_uint, c_ulong};

#[repr(C)]
pub struct riscv_hwprobe {
    pub key: c_int,
    pub value: c_ulong,
}

#[repr(C)]
pub struct vdso_arch_data {
    pub homogeneous_cpus: bool,
    pub all_cpu_hwprobe_values: [c_ulong; 0],
}

extern "C" {
    static vdso_u_arch_data: vdso_arch_data;

    fn riscv_hwprobe(
        pairs: *mut riscv_hwprobe,
        pair_count: usize,
        cpusetsize: usize,
        cpus: *mut c_ulong,
        flags: c_uint,
    ) -> c_int;

    fn riscv_hwprobe_key_is_valid(key: c_int) -> bool;
    fn riscv_hwprobe_pair_cmp(a: *const riscv_hwprobe, b: *const riscv_hwprobe) -> c_int;
}

const EINVAL: c_int = 22;
const RISCV_HWPROBE_WHICH_CPUS: c_uint = 1;

unsafe fn riscv_vdso_get_values(
    pairs: *mut riscv_hwprobe,
    pair_count: usize,
    cpusetsize: usize,
    cpus: *mut c_ulong,
    flags: c_uint,
) -> c_int {
    let avd = &vdso_u_arch_data;
    let all_cpus = cpusetsize == 0 && cpus.is_null();
    let mut p = pairs;
    let end = pairs.add(pair_count);

    /*
     * Defer to the syscall for exotic requests. The vdso has answers
     * stashed away only for the "all cpus" case. If all CPUs are
     * homogeneous, then this function can handle requests for arbitrary
     * masks.
     */
    if flags != 0 || (!all_cpus && !avd.homogeneous_cpus) {
        return riscv_hwprobe(pairs, pair_count, cpusetsize, cpus, flags);
    }

    /* This is something we can handle, fill out the pairs. */
    while p < end {
        if riscv_hwprobe_key_is_valid((*p).key) {
            (*p).value = avd.all_cpu_hwprobe_values[(*p).key as usize];
        } else {
            (*p).key = -1;
            (*p).value = 0;
        }

        p = p.add(1);
    }

    0
}

unsafe fn riscv_vdso_get_cpus(
    pairs: *mut riscv_hwprobe,
    pair_count: usize,
    cpusetsize: usize,
    cpus: *mut c_ulong,
    flags: c_uint,
) -> c_int {
    let avd = &vdso_u_arch_data;
    let mut p = pairs;
    let end = pairs.add(pair_count);
    let c = cpus as *mut u8;
    let mut empty_cpus = true;
    let mut clear_all = false;

    if cpusetsize == 0 || cpus.is_null() {
        return -EINVAL;
    }

    for i in 0..cpusetsize {
        if *c.add(i) != 0 {
            empty_cpus = false;
            break;
        }
    }

    if empty_cpus || flags != RISCV_HWPROBE_WHICH_CPUS || !avd.homogeneous_cpus {
        return riscv_hwprobe(pairs, pair_count, cpusetsize, cpus, flags);
    }

    while p < end {
        if riscv_hwprobe_key_is_valid((*p).key) {
            let t = riscv_hwprobe {
                key: (*p).key,
                value: avd.all_cpu_hwprobe_values[(*p).key as usize],
            };

            if riscv_hwprobe_pair_cmp(&t, p) == 0 {
                clear_all = true;
            }
        } else {
            clear_all = true;
            (*p).key = -1;
            (*p).value = 0;
        }
        p = p.add(1);
    }

    if clear_all {
        for i in 0..cpusetsize {
            *c.add(i) = 0;
        }
    }

    0
}

/* Add a prototype to avoid -Wmissing-prototypes warning. */
pub unsafe fn __vdso_riscv_hwprobe(
    pairs: *mut riscv_hwprobe,
    pair_count: usize,
    cpusetsize: usize,
    cpus: *mut c_ulong,
    flags: c_uint,
) -> c_int;

pub unsafe fn __vdso_riscv_hwprobe(
    pairs: *mut riscv_hwprobe,
    pair_count: usize,
    cpusetsize: usize,
    cpus: *mut c_ulong,
    flags: c_uint,
) -> c_int {
    if flags & RISCV_HWPROBE_WHICH_CPUS != 0 {
        return riscv_vdso_get_cpus(pairs, pair_count, cpusetsize, cpus, flags);
    }

    riscv_vdso_get_values(pairs, pair_count, cpusetsize, cpus, flags)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
