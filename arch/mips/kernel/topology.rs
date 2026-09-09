// SPDX-License-Identifier: GPL-2.0
//
// Dependencies supplied by the Linux kernel are intentionally left external.

use core::ffi::c_int;

#[repr(C)]
pub struct cpu {
    pub hotpluggable: bool,
}

// Equivalent to DEFINE_PER_CPU(struct cpu, cpu_devices).
extern "C" {
    static mut cpu_devices: cpu;
}

extern "C" {
    fn register_cpu(c: *mut cpu, cpu: c_int) -> c_int;
    fn printk(level_and_format: *const u8, ...) -> c_int;
}

// The kernel's per-CPU accessor and present-CPU iterator are provided by the
// surrounding build environment.
extern "Rust" {
    fn per_cpu_cpu_devices(cpu: c_int) -> *mut cpu;
}

#[allow(non_upper_case_globals)]
static KERN_WARNING: &[u8] = b"topology_init: register_cpu %d failed (%d)\n\0";

#[allow(non_snake_case)]
pub unsafe fn topology_init() -> c_int {
    let mut i: c_int;
    let mut ret: c_int;

    // Equivalent to for_each_present_cpu(i), supplied by the kernel.
    for_each_present_cpu!(i, {
        let c: *mut cpu = per_cpu_cpu_devices(i);

        (*c).hotpluggable = i != 0;
        ret = register_cpu(c, i);
        if ret != 0 {
            printk(KERN_WARNING.as_ptr(), i, ret);
        }
    });

    0
}

// Equivalent to subsys_initcall(topology_init).
subsys_initcall!(topology_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
